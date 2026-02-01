use sea_orm_migration::prelude::*;

use crate::m000004_create_boxes::Boxes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tokens::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tokens::TokenId).text().not_null())
                    .col(ColumnDef::new(Tokens::TokenName).text().not_null())
                    .col(ColumnDef::new(Tokens::TokenDescription).text().not_null())
                    .col(ColumnDef::new(Tokens::IssuerBox).big_integer().not_null())
                    .col(ColumnDef::new(Tokens::IssuanceBox).big_integer().not_null())
                    .col(ColumnDef::new(Tokens::Decimals).integer().null())
                    .col(ColumnDef::new(Tokens::Minted).big_integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("tokens__boxes_fk")
                            .from(Tokens::Table, Tokens::IssuanceBox)
                            .to(Boxes::Table, Boxes::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("fki_tokens__boxes_fk")
                    .table(Tokens::Table)
                    .col(Tokens::IssuanceBox)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idz_tokens_token_id")
                    .table(Tokens::Table)
                    .col(Tokens::TokenId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tokens::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Tokens {
    Table,
    Id,
    TokenId,
    TokenName,
    TokenDescription,
    IssuerBox,
    IssuanceBox,
    Decimals,
    Minted,
}
