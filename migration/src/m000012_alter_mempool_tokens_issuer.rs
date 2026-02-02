use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Mempool is transient data, so we can truncate and alter safely
        manager
            .get_connection()
            .execute_unprepared("TRUNCATE TABLE mempool_tokens CASCADE")
            .await?;

        // Drop the foreign key constraint on issuer_box
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE mempool_tokens DROP CONSTRAINT mempool_tokens_issuer_fk",
            )
            .await?;

        // Change issuer_box from BIGINT to TEXT
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE mempool_tokens ALTER COLUMN issuer_box TYPE TEXT USING issuer_box::TEXT",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("TRUNCATE TABLE mempool_tokens CASCADE")
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE mempool_tokens ALTER COLUMN issuer_box TYPE BIGINT USING issuer_box::BIGINT",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE mempool_tokens ADD CONSTRAINT mempool_tokens_issuer_fk FOREIGN KEY (issuer_box) REFERENCES mempool_boxes(id) ON DELETE CASCADE",
            )
            .await?;

        Ok(())
    }
}
