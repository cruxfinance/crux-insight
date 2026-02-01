use sea_orm_migration::prelude::*;

use crate::m000003_create_transactions::Transactions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create inputs table
        manager
            .create_table(
                Table::create()
                    .table(Inputs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Inputs::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Inputs::BoxId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Inputs::TransactionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Inputs::SpendingProof)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Inputs::Index).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("inputs__transactions_fk")
                            .from(Inputs::Table, Inputs::TransactionId)
                            .to(Transactions::Table, Transactions::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("fki_inputs__transactions_fk")
                    .table(Inputs::Table)
                    .col(Inputs::TransactionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("inputs_box_id_idx")
                    .table(Inputs::Table)
                    .col(Inputs::BoxId)
                    .to_owned(),
            )
            .await?;

        // Create data_inputs table
        manager
            .create_table(
                Table::create()
                    .table(DataInputs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DataInputs::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DataInputs::BoxId).big_integer().not_null())
                    .col(
                        ColumnDef::new(DataInputs::TransactionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DataInputs::Index).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DataInputs::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Inputs::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Inputs {
    Table,
    Id,
    BoxId,
    TransactionId,
    SpendingProof,
    Index,
}

#[derive(Iden)]
pub enum DataInputs {
    Table,
    Id,
    BoxId,
    TransactionId,
    Index,
}
