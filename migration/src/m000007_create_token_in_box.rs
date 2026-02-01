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
                    .table(TokenInBox::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TokenInBox::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TokenInBox::BoxId).big_integer().not_null())
                    .col(ColumnDef::new(TokenInBox::TokenId).big_integer().not_null())
                    .col(ColumnDef::new(TokenInBox::Amount).big_integer().not_null())
                    .col(ColumnDef::new(TokenInBox::Index).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("token_in_box__boxes_fk")
                            .from(TokenInBox::Table, TokenInBox::BoxId)
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
                    .name("fki_token_in_box__boxes_fk")
                    .table(TokenInBox::Table)
                    .col(TokenInBox::BoxId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_token_in_box_box_id")
                    .table(TokenInBox::Table)
                    .col(TokenInBox::BoxId)
                    .to_owned(),
            )
            .await?;

        // Index with INCLUDE requires raw SQL
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_token_in_box_token_id ON token_in_box (token_id) INCLUDE (box_id)",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TokenInBox::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum TokenInBox {
    Table,
    Id,
    BoxId,
    TokenId,
    Amount,
    Index,
}
