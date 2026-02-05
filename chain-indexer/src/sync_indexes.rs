use sea_orm::{ConnectionTrait, DatabaseConnection, FromQueryResult};
use tracing::info;

/// Tables that crux-insight writes to during sync
const SYNC_TABLES: &[&str] = &[
    "blocks",
    "addresses",
    "transactions",
    "boxes",
    "tokens",
    "inputs",
    "token_in_box",
];

/// Indexes to keep during sync (needed for cache-miss lookups)
const KEEP_INDEXES: &[&str] = &[
    "idx_boxes_box_id",       // box lookup by box_id
    "idx_addresses_ergotree", // address lookup by ergotree
    "idz_tokens_token_id",    // token lookup by token_id
];

#[derive(Debug, FromQueryResult)]
struct IndexDef {
    indexname: String,
    indexdef: String,
}

#[derive(Debug, FromQueryResult)]
struct FkDef {
    conname: String,
    table_name: String,
    definition: String,
}

/// Saved index and FK definitions for later recreation
pub struct SavedIndexes {
    indexes: Vec<IndexDef>,
    foreign_keys: Vec<FkDef>,
}

/// Query the database for all non-PK, non-unique-kept indexes and all FK constraints
/// on sync tables, save their definitions, then drop them.
pub async fn drop_sync_indexes(db: &DatabaseConnection) -> SavedIndexes {
    // Find all non-primary, non-kept indexes on sync tables
    let table_list = SYNC_TABLES
        .iter()
        .map(|t| format!("'{}'", t))
        .collect::<Vec<_>>()
        .join(",");

    let keep_list = KEEP_INDEXES
        .iter()
        .map(|i| format!("'{}'", i))
        .collect::<Vec<_>>()
        .join(",");

    let index_query = format!(
        "SELECT indexname, indexdef FROM pg_indexes
         WHERE schemaname = 'public'
         AND tablename IN ({})
         AND indexname NOT IN ({})
         AND indexname NOT LIKE '%_pkey'",
        table_list, keep_list
    );

    let indexes: Vec<IndexDef> = IndexDef::find_by_statement(sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        index_query,
    ))
    .all(db)
    .await
    .unwrap_or_default();

    info!("Found {} non-essential indexes to drop", indexes.len());

    // Find all FK constraints on sync tables
    let fk_query = format!(
        "SELECT c.conname, t.relname as table_name,
         pg_get_constraintdef(c.oid) as definition
         FROM pg_constraint c
         JOIN pg_class t ON c.conrelid = t.oid
         JOIN pg_namespace n ON t.relnamespace = n.oid
         WHERE c.contype = 'f'
         AND n.nspname = 'public'
         AND t.relname IN ({})",
        table_list
    );

    let foreign_keys: Vec<FkDef> = FkDef::find_by_statement(sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        fk_query,
    ))
    .all(db)
    .await
    .unwrap_or_default();

    info!(
        "Found {} foreign key constraints to drop",
        foreign_keys.len()
    );

    // Drop FK constraints first (some indexes support FKs)
    for fk in &foreign_keys {
        let sql = format!(
            "ALTER TABLE public.{} DROP CONSTRAINT IF EXISTS {}",
            fk.table_name, fk.conname
        );
        info!("Dropping FK: {}", fk.conname);
        let _ = db.execute_unprepared(&sql).await;
    }

    // Drop indexes
    for idx in &indexes {
        let sql = format!("DROP INDEX IF EXISTS public.{}", idx.indexname);
        info!("Dropping index: {}", idx.indexname);
        let _ = db.execute_unprepared(&sql).await;
    }

    SavedIndexes {
        indexes,
        foreign_keys,
    }
}

/// Recreate all previously dropped indexes and FK constraints.
pub async fn recreate_sync_indexes(db: &DatabaseConnection, saved: &SavedIndexes) {
    info!(
        "Recreating {} indexes and {} foreign keys",
        saved.indexes.len(),
        saved.foreign_keys.len()
    );

    // Recreate indexes (use CONCURRENTLY to avoid blocking reads, but not inside a transaction)
    for idx in &saved.indexes {
        info!("Recreating index: {}", idx.indexname);
        if let Err(e) = db.execute_unprepared(&idx.indexdef).await {
            panic!("Failed to recreate index {}: {}", idx.indexname, e);
        }
    }

    // Recreate FK constraints
    for fk in &saved.foreign_keys {
        let sql = format!(
            "ALTER TABLE public.{} ADD CONSTRAINT {} {}",
            fk.table_name, fk.conname, fk.definition
        );
        info!("Recreating FK: {}", fk.conname);
        if let Err(e) = db.execute_unprepared(&sql).await {
            panic!("Failed to recreate FK {}: {}", fk.conname, e);
        }
    }

    info!("All indexes and foreign keys recreated");
}
