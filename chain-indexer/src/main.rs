#![deny(unused_crate_dependencies)]

extern crate reqwest;

pub mod actors;
mod database;
pub mod entities;
mod settings;
pub mod types;

use anyhow::Result;
use chrono::NaiveDateTime;
use entities::blocks;
use ergo_lib::ergotree_ir::chain::address::{Address, NetworkAddress, NetworkPrefix};
use ergo_lib::ergotree_ir::ergo_tree::ErgoTree;
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;
use ergo_node_client::apis::configuration::Configuration;
use ergo_node_client::apis::{info_api, utxo_api};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder};
use sea_orm::{ConnectionTrait, Set};
use tokio::sync::mpsc::{self};
use tracing::{info, instrument, Level};
use tracing_subscriber::FmtSubscriber;

use crate::actors::data_inserter::insert_data;
use crate::actors::header_fetcher::fetch_headers;
use crate::actors::transaction_fetcher::fetch_transactions;
use crate::database::CIDatabase;
use crate::settings::Settings;

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new().unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("Creating channels");
    let (header_tx, header_rx) = mpsc::channel(32);
    let (blockchain_data_tx, blockchain_data_rx) = mpsc::channel(32);

    info!("Node configuration");
    let node_conf = Configuration {
        base_path: settings.ergo_node.url.to_owned(),
        ..Default::default()
    };
    info!("Connecting to db");
    let db = CIDatabase {
        settings: settings.to_owned(),
    }
    .connect()
    .await;

    info!("Getting current max db height");
    let mut current_max_db_height = blocks::Entity::find()
        .order_by_desc(blocks::Column::Height)
        .one(&db)
        .await?
        .map_or_else(|| -1, |b| b.height);
    info!("{}", current_max_db_height);

    info!("Getting current node height");
    let current_max_node_height = info_api::get_node_info(&node_conf)
        .await?
        .full_height
        .expect("Could not fetch full height from node");
    info!("{}", current_max_node_height);

    if current_max_db_height == -1 {
        info!("Inserting genesis block");
        let block = entities::blocks::ActiveModel {
            height: Set(0),
            header_id: Set(
                "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            ),
            timestamp: Set(NaiveDateTime::from_timestamp_millis(1561978877137)
                .expect("Expected NaiveDateTime")),
            header: Set(serde_json::json!({})),
        };
        block.insert(&db).await?;
        info!("Fetching genesis utxos");
        let gen_utxos = utxo_api::genesis_boxes(&node_conf).await?;
        let transaction = entities::transactions::ActiveModel {
            id: Set(0),
            transaction_id: Set(gen_utxos[0].transaction_id.to_owned().unwrap()),
            height: Set(0),
            ..Default::default()
        };
        let transaction_in_db = transaction.insert(&db).await?;
        let mut index = 0;

        for utxo in gen_utxos.iter() {
            let address = entities::addresses::ActiveModel {
                id: Set(index),
                address: Set(NetworkAddress::new(
                    NetworkPrefix::Mainnet,
                    &Address::recreate_from_ergo_tree(
                        &ErgoTree::sigma_parse_bytes(
                            &hex::decode(utxo.ergo_tree.to_owned()).unwrap(),
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                )
                .to_base58()),
                ergotree: Set(utxo.ergo_tree.to_owned()),
                ergotree_hash: Set("".to_string()),
                ..Default::default()
            };
            let address_in_db = address.insert(&db).await?;
            let txo = entities::boxes::ActiveModel {
                id: Set(index),
                address: Set(address_in_db.id),
                created: Set(0),
                data: Set(serde_json::json!(utxo)),
                box_id: Set(utxo.box_id.to_owned().unwrap().to_string()),
                transaction_id: Set(Some(transaction_in_db.id)),
                ..Default::default()
            };
            index += 1;
            txo.insert(&db).await?;
        }

        info!("Inserting erg into tokens table");
        let erg_token = entities::tokens::ActiveModel {
            id: Set(0),
            token_id: Set(
                "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            ),
            token_name: Set("erg".to_string()),
            token_description: Set("Ergo coin".to_string()),
            issuer_box: Set(0),
            issuance_box: Set(0),
            decimals: Set(Some(9)),
            minted: Set(Some(97739924000000000)),
        };
        erg_token.insert(&db).await?;
        current_max_db_height = 0;
        let _ = db
            .execute_unprepared(
                "
            alter table public.asset_in_box set unlogged;
            alter table public.token_in_box set unlogged;
            alter table public.wrapped set unlogged;
            alter table public.tokens set unlogged;
            alter table public.boxes set unlogged;
            alter table public.inputs set unlogged;
            alter table public.transactions set unlogged;
            alter table public.blocks set unlogged;
            alter table public.addresses set unlogged;
        ",
            )
            .await;
    }

    let thr = tokio::spawn(async move {
        insert_data(blockchain_data_rx).await;
    });

    let node_conf_tx = node_conf.clone();
    let blockchain_data_tx_cl = blockchain_data_tx.clone();
    tokio::spawn(async move {
        fetch_transactions(&node_conf_tx, header_rx, blockchain_data_tx_cl).await;
    });

    tokio::spawn(async move {
        fetch_headers(
            &node_conf,
            current_max_db_height,
            current_max_node_height,
            header_tx,
        )
        .await;
    });

    let _ = thr.await;

    Ok(())
}
