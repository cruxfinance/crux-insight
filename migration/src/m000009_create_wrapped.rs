use sea_orm_migration::prelude::*;

use crate::m000004_create_boxes::Boxes;
use crate::m000005_create_tokens::Tokens;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wrapped::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Wrapped::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Wrapped::BoxId).big_integer().not_null())
                    .col(ColumnDef::new(Wrapped::TokenId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Wrapped::WrappedTokenId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Wrapped::WrappedAmount).double().not_null())
                    .col(ColumnDef::new(Wrapped::Type).text().not_null())
                    .col(ColumnDef::new(Wrapped::SubType).text().null())
                    .col(ColumnDef::new(Wrapped::Timepoint).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("wrapped_box_id_fkey")
                            .from(Wrapped::Table, Wrapped::BoxId)
                            .to(Boxes::Table, Boxes::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("wrapped_token_id_fkey")
                            .from(Wrapped::Table, Wrapped::TokenId)
                            .to(Tokens::Table, Tokens::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("wrapped_wrapped_token_id_fkey")
                            .from(Wrapped::Table, Wrapped::WrappedTokenId)
                            .to(Tokens::Table, Tokens::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_token_id_box_id")
                    .table(Wrapped::Table)
                    .col((Wrapped::TokenId, IndexOrder::Asc))
                    .col((Wrapped::BoxId, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_wrapped_type_sub_type")
                    .table(Wrapped::Table)
                    .col(Wrapped::Type)
                    .col(Wrapped::SubType)
                    .col(Wrapped::Timepoint)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("wrapped_token_id_idx")
                    .table(Wrapped::Table)
                    .col(Wrapped::TokenId)
                    .col(Wrapped::Timepoint)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wrapped::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Wrapped {
    Table,
    Id,
    BoxId,
    TokenId,
    WrappedTokenId,
    WrappedAmount,
    Type,
    SubType,
    Timepoint,
}
