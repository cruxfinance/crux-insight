use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Blocks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Blocks::Height)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Blocks::HeaderId).text().not_null())
                    .col(ColumnDef::new(Blocks::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(Blocks::Header).json_binary().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_blocks_timestamp")
                    .table(Blocks::Table)
                    .col(Blocks::Timestamp)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Blocks::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Blocks {
    Table,
    Height,
    HeaderId,
    Timestamp,
    Header,
}
