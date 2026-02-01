use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Addresses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Addresses::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Addresses::Address).text().not_null())
                    .col(ColumnDef::new(Addresses::Ergotree).text().not_null())
                    .col(ColumnDef::new(Addresses::ErgotreeHash).text().not_null())
                    .col(ColumnDef::new(Addresses::ErgotreeTemplate).text().null())
                    .to_owned(),
            )
            .await?;

        // Hash index on address for fast lookups
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_addresses_address ON addresses USING hash (address)",
            )
            .await?;

        // Hash index on ergotree
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_addresses_ergotree ON addresses USING hash (ergotree)",
            )
            .await?;

        // Hash index on ergotree_template
        manager
            .get_connection()
            .execute_unprepared("CREATE INDEX addresses_ergotree_template_idx ON addresses USING hash (ergotree_template)")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Addresses::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Addresses {
    Table,
    Id,
    Address,
    Ergotree,
    ErgotreeHash,
    ErgotreeTemplate,
}
