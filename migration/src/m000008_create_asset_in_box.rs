use sea_orm_migration::prelude::*;

use crate::m000001_create_blocks::Blocks;
use crate::m000004_create_boxes::Boxes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AssetInBox::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AssetInBox::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AssetInBox::BoxId).big_integer().not_null())
                    .col(
                        ColumnDef::new(AssetInBox::TransactionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AssetInBox::TokenId).big_integer().not_null())
                    .col(
                        ColumnDef::new(AssetInBox::TokenAmount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AssetInBox::Address).big_integer().not_null())
                    .col(ColumnDef::new(AssetInBox::Created).integer().not_null())
                    .col(ColumnDef::new(AssetInBox::Spent).integer().null())
                    .col(ColumnDef::new(AssetInBox::Index).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("asset_in_box_fk")
                            .from(AssetInBox::Table, AssetInBox::BoxId)
                            .to(Boxes::Table, Boxes::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("asset_in_box_spent_fk")
                            .from(AssetInBox::Table, AssetInBox::Spent)
                            .to(Blocks::Table, Blocks::Height)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("asset_in_box_address_idx")
                    .table(AssetInBox::Table)
                    .col(AssetInBox::Address)
                    .col(AssetInBox::Spent)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("asset_in_box_spent_idx")
                    .table(AssetInBox::Table)
                    .col(AssetInBox::Spent)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("asset_in_box_token_id_idx")
                    .table(AssetInBox::Table)
                    .col(AssetInBox::TokenId)
                    .col(AssetInBox::Spent)
                    .col(AssetInBox::Created)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("asset_in_box_transaction_id_idx")
                    .table(AssetInBox::Table)
                    .col(AssetInBox::TransactionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_asset_in_box__box_id")
                    .table(AssetInBox::Table)
                    .col(AssetInBox::BoxId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AssetInBox::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum AssetInBox {
    Table,
    Id,
    BoxId,
    TransactionId,
    TokenId,
    TokenAmount,
    Address,
    Created,
    Spent,
    Index,
}
