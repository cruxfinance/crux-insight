use sea_orm_migration::prelude::*;

use crate::m000001_create_blocks::Blocks;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transactions::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Transactions::TransactionId)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Transactions::Height).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("transactions__blocks_fk")
                            .from(Transactions::Table, Transactions::Height)
                            .to(Blocks::Table, Blocks::Height)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("fki_transactions__blocks_fk")
                    .table(Transactions::Table)
                    .col(Transactions::Height)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Transactions {
    Table,
    Id,
    TransactionId,
    Height,
}
