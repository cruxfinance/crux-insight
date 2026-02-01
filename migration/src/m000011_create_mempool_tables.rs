use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create mempool_transactions table
        manager
            .create_table(
                Table::create()
                    .table(MempoolTransactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MempoolTransactions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MempoolTransactions::TransactionId)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(MempoolTransactions::Data)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MempoolTransactions::FirstSeen)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_mempool_tx_id ON mempool_transactions USING hash (transaction_id)",
            )
            .await?;

        // Create mempool_boxes table
        manager
            .create_table(
                Table::create()
                    .table(MempoolBoxes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MempoolBoxes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MempoolBoxes::BoxId)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(MempoolBoxes::Address).big_integer().null())
                    .col(ColumnDef::new(MempoolBoxes::Ergotree).text().not_null())
                    .col(ColumnDef::new(MempoolBoxes::Data).json_binary().not_null())
                    .col(
                        ColumnDef::new(MempoolBoxes::MempoolTransactionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MempoolBoxes::OutputIndex)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("mempool_boxes_tx_fk")
                            .from(MempoolBoxes::Table, MempoolBoxes::MempoolTransactionId)
                            .to(MempoolTransactions::Table, MempoolTransactions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_mempool_boxes_box_id ON mempool_boxes USING hash (box_id)",
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_mempool_boxes_address")
                    .table(MempoolBoxes::Table)
                    .col(MempoolBoxes::Address)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_mempool_boxes_tx_id")
                    .table(MempoolBoxes::Table)
                    .col(MempoolBoxes::MempoolTransactionId)
                    .to_owned(),
            )
            .await?;

        // Create mempool_inputs table
        manager
            .create_table(
                Table::create()
                    .table(MempoolInputs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MempoolInputs::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MempoolInputs::MempoolTransactionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MempoolInputs::BoxId).text().not_null())
                    .col(
                        ColumnDef::new(MempoolInputs::ConfirmedBoxId)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(MempoolInputs::SpendingProof)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MempoolInputs::Index).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("mempool_inputs_tx_fk")
                            .from(MempoolInputs::Table, MempoolInputs::MempoolTransactionId)
                            .to(MempoolTransactions::Table, MempoolTransactions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_mempool_inputs_box_id ON mempool_inputs USING hash (box_id)",
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_mempool_inputs_tx_id")
                    .table(MempoolInputs::Table)
                    .col(MempoolInputs::MempoolTransactionId)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_mempool_inputs_confirmed ON mempool_inputs (confirmed_box_id) WHERE confirmed_box_id IS NOT NULL",
            )
            .await?;

        // Create mempool_tokens table
        manager
            .create_table(
                Table::create()
                    .table(MempoolTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MempoolTokens::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MempoolTokens::TokenId)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(MempoolTokens::TokenName).text().null())
                    .col(
                        ColumnDef::new(MempoolTokens::TokenDescription)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(MempoolTokens::Decimals).integer().null())
                    .col(ColumnDef::new(MempoolTokens::Minted).big_integer().null())
                    .col(
                        ColumnDef::new(MempoolTokens::IssuerBox)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MempoolTokens::IssuanceBox)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("mempool_tokens_issuer_fk")
                            .from(MempoolTokens::Table, MempoolTokens::IssuerBox)
                            .to(MempoolBoxes::Table, MempoolBoxes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("mempool_tokens_issuance_fk")
                            .from(MempoolTokens::Table, MempoolTokens::IssuanceBox)
                            .to(MempoolBoxes::Table, MempoolBoxes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_mempool_tokens_token_id ON mempool_tokens USING hash (token_id)",
            )
            .await?;

        // Create mempool_token_in_box table
        manager
            .create_table(
                Table::create()
                    .table(MempoolTokenInBox::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MempoolTokenInBox::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MempoolTokenInBox::MempoolBoxId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MempoolTokenInBox::TokenId)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(MempoolTokenInBox::MempoolTokenId)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(MempoolTokenInBox::TokenIdStr)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MempoolTokenInBox::Amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MempoolTokenInBox::Index)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("mempool_tib_box_fk")
                            .from(MempoolTokenInBox::Table, MempoolTokenInBox::MempoolBoxId)
                            .to(MempoolBoxes::Table, MempoolBoxes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_mempool_tib_box")
                    .table(MempoolTokenInBox::Table)
                    .col(MempoolTokenInBox::MempoolBoxId)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_mempool_tib_token_str ON mempool_token_in_box USING hash (token_id_str)",
            )
            .await?;

        // Add check constraint
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE mempool_token_in_box ADD CONSTRAINT chk_mempool_token_ref CHECK (token_id IS NOT NULL OR mempool_token_id IS NOT NULL)",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MempoolTokenInBox::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MempoolTokens::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MempoolInputs::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MempoolBoxes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MempoolTransactions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum MempoolTransactions {
    Table,
    Id,
    TransactionId,
    Data,
    FirstSeen,
}

#[derive(Iden)]
pub enum MempoolBoxes {
    Table,
    Id,
    BoxId,
    Address,
    Ergotree,
    Data,
    MempoolTransactionId,
    OutputIndex,
}

#[derive(Iden)]
pub enum MempoolInputs {
    Table,
    Id,
    MempoolTransactionId,
    BoxId,
    ConfirmedBoxId,
    SpendingProof,
    Index,
}

#[derive(Iden)]
pub enum MempoolTokens {
    Table,
    Id,
    TokenId,
    TokenName,
    TokenDescription,
    Decimals,
    Minted,
    IssuerBox,
    IssuanceBox,
}

#[derive(Iden)]
pub enum MempoolTokenInBox {
    Table,
    Id,
    MempoolBoxId,
    TokenId,
    MempoolTokenId,
    TokenIdStr,
    Amount,
    Index,
}
