pub use sea_orm_migration::prelude::*;

mod m000001_create_blocks;
mod m000002_create_addresses;
mod m000003_create_transactions;
mod m000004_create_boxes;
mod m000005_create_tokens;
mod m000006_create_inputs;
mod m000007_create_token_in_box;
mod m000008_create_asset_in_box;
mod m000009_create_wrapped;
mod m000010_create_functions;
mod m000011_create_mempool_tables;
mod m000012_alter_mempool_tokens_issuer;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000001_create_blocks::Migration),
            Box::new(m000002_create_addresses::Migration),
            Box::new(m000003_create_transactions::Migration),
            Box::new(m000004_create_boxes::Migration),
            Box::new(m000005_create_tokens::Migration),
            Box::new(m000006_create_inputs::Migration),
            Box::new(m000007_create_token_in_box::Migration),
            Box::new(m000008_create_asset_in_box::Migration),
            Box::new(m000009_create_wrapped::Migration),
            Box::new(m000010_create_functions::Migration),
            Box::new(m000011_create_mempool_tables::Migration),
            Box::new(m000012_alter_mempool_tokens_issuer::Migration),
        ]
    }
}
