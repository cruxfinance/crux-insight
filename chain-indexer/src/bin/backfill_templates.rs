//! One-shot backfill of `public.addresses.ergotree_template`.
//!
//! Before the ergo-lib 0.29 upgrade the indexer stored an empty template for
//! every ErgoTree v3 (Ergo 6.0) contract, because `template_bytes()` failed on
//! them. Downstream consumers (ci-modules, ci-api) classify contracts by this
//! column, so those rows must be recomputed.
//!
//! Run this ONCE after the 0.29 indexer is deployed (new addresses are then
//! correct on insert). It is idempotent and safe to interrupt and re-run:
//! rows are selected by keyset pagination on `id`, each batch is committed in
//! its own transaction, and only rows whose template is empty or NULL and now
//! computable are updated. No other table is touched.
//!
//! Usage: `backfill_templates [--dry-run] [--batch-size N]`
//! Database and settings come from the same config / `POSTGRES_*` env vars as
//! the indexer itself.

#![allow(dead_code)]

#[path = "../database.rs"]
mod database;
#[path = "../ergotree_template.rs"]
mod ergotree_template;
#[path = "../settings.rs"]
mod settings;

use std::collections::BTreeMap;

use anyhow::Result;
use ergo_lib::ergotree_ir::ergo_tree::ErgoTree;
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;
use sea_orm::{ConnectionTrait, DatabaseBackend, FromQueryResult, Statement, TransactionTrait};

use crate::database::CIDatabase;
use crate::ergotree_template::template_hex_for_tree;
use crate::settings::Settings;

#[derive(Debug, FromQueryResult)]
struct Row {
    id: i64,
    ergotree: String,
}

#[derive(Default)]
struct Totals {
    scanned: u64,
    computable: u64,
    failing: u64,
    updated: u64,
    by_header: BTreeMap<u8, u64>,
    failing_examples: Vec<(i64, String, Option<u8>)>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dry_run = args.iter().any(|a| a == "--dry-run");
    let batch_size: i64 = args
        .iter()
        .position(|a| a == "--batch-size")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(5_000);

    let settings = Settings::new()?;
    let db = CIDatabase { settings }.connect().await;

    println!(
        "backfill_templates: dry_run={dry_run} batch_size={batch_size}"
    );

    let mut totals = Totals::default();
    let mut last_id: i64 = -1;
    loop {
        let rows = Row::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT id, ergotree FROM public.addresses \
             WHERE id > $1 AND (ergotree_template = '' OR ergotree_template IS NULL) \
             ORDER BY id LIMIT $2",
            [last_id.into(), batch_size.into()],
        ))
        .all(&db)
        .await?;
        if rows.is_empty() {
            break;
        }
        last_id = rows.last().unwrap().id;

        let mut updates: Vec<(i64, String)> = Vec::new();
        for row in &rows {
            totals.scanned += 1;
            let bytes = hex::decode(&row.ergotree).unwrap_or_default();
            let tpl = ErgoTree::sigma_parse_bytes(&bytes)
                .ok()
                .map(|t| template_hex_for_tree(&t))
                .unwrap_or_default();
            if tpl.is_empty() {
                totals.failing += 1;
                if totals.failing_examples.len() < 10 {
                    let head: String = row.ergotree.chars().take(40).collect();
                    totals.failing_examples.push((row.id, head, bytes.first().copied()));
                }
            } else {
                totals.computable += 1;
                *totals.by_header.entry(bytes[0]).or_default() += 1;
                updates.push((row.id, tpl));
            }
        }

        if !dry_run && !updates.is_empty() {
            let txn = db.begin().await?;
            for (id, tpl) in &updates {
                txn.execute(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    "UPDATE public.addresses SET ergotree_template = $1 \
                     WHERE id = $2 AND (ergotree_template = '' OR ergotree_template IS NULL)",
                    [tpl.clone().into(), (*id).into()],
                ))
                .await?;
            }
            txn.commit().await?;
            totals.updated += updates.len() as u64;
        }

        println!(
            "batch up to id {last_id}: scanned={} computable={} failing={} updated={}",
            totals.scanned, totals.computable, totals.failing, totals.updated
        );
    }

    println!("---- summary{} ----", if dry_run { " (dry run, no writes)" } else { "" });
    println!("empty/NULL rows scanned : {}", totals.scanned);
    println!("now computable          : {}", totals.computable);
    println!("still failing           : {}", totals.failing);
    println!("rows updated            : {}", totals.updated);
    println!("computable by header byte:");
    for (h, n) in &totals.by_header {
        println!("  0x{h:02x}: {n}");
    }
    if !totals.failing_examples.is_empty() {
        println!("failing examples (id, first 20 bytes hex, header byte):");
        for (id, head, hb) in &totals.failing_examples {
            match hb {
                Some(h) => println!("  id={id} header=0x{h:02x} tree={head}"),
                None => println!("  id={id} header=<unparseable hex> tree={head}"),
            }
        }
    }
    Ok(())
}
