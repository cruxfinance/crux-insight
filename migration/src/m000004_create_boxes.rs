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
                    .table(Boxes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Boxes::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Boxes::Address).big_integer().not_null())
                    .col(ColumnDef::new(Boxes::Data).json_binary().not_null())
                    .col(ColumnDef::new(Boxes::Created).integer().not_null())
                    .col(ColumnDef::new(Boxes::Spent).integer().null())
                    .col(ColumnDef::new(Boxes::BoxId).text().not_null())
                    .col(ColumnDef::new(Boxes::TransactionId).big_integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_boxes_created")
                            .from(Boxes::Table, Boxes::Created)
                            .to(Blocks::Table, Blocks::Height)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_boxes_spent")
                            .from(Boxes::Table, Boxes::Spent)
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
                    .name("boxes_spent_idx")
                    .table(Boxes::Table)
                    .col(Boxes::Spent)
                    .col(Boxes::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("fki_fk_boxes_created")
                    .table(Boxes::Table)
                    .col(Boxes::Created)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("fki_fk_boxes_spent")
                    .table(Boxes::Table)
                    .col(Boxes::Spent)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_boxes__transaction")
                    .table(Boxes::Table)
                    .col(Boxes::TransactionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_boxes_address_box_id")
                    .table(Boxes::Table)
                    .col(Boxes::Address)
                    .col(Boxes::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_boxes_address_created_spent")
                    .table(Boxes::Table)
                    .col(Boxes::Address)
                    .col(Boxes::Created)
                    .col(Boxes::Spent)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_boxes_box_id")
                    .table(Boxes::Table)
                    .col(Boxes::BoxId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_boxes_box_id_spent")
                    .table(Boxes::Table)
                    .col(Boxes::BoxId)
                    .col(Boxes::Spent)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_id_created_spent")
                    .table(Boxes::Table)
                    .col(Boxes::Id)
                    .col(Boxes::Created)
                    .col(Boxes::Spent)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Boxes::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Boxes {
    Table,
    Id,
    Address,
    Data,
    Created,
    Spent,
    BoxId,
    TransactionId,
}
