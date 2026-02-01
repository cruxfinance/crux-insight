use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create custom type
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TYPE token AS (
                    amount bigint,
                    "tokenId" text
                )
                "#,
            )
            .await?;

        // Create get_wrapped_tokens function
        // Note: This calls get_token_info which will be created by ci-modules
        // The function will fail until ci-modules migrations run
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE FUNCTION get_wrapped_tokens(t_id bigint, t_amount bigint)
                RETURNS TABLE(wrapped_tokens json)
                LANGUAGE plpgsql STABLE PARALLEL SAFE
                AS $$
                BEGIN
                RETURN QUERY
                select T.token_info from (
                select get_token_info(wrapped_token_id, round(t_amount*wrapped_amount)::bigint) as token_info
                from public.wrapped
                where token_id = t_id
                and box_id = (select max(box_id) from public.wrapped where token_id = t_id)
                and timepoint = (select max(timepoint) from public.wrapped where token_id = t_id)
                ) T;
                END;
                $$
                "#,
            )
            .await?;

        // Create get_token_supply function
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE FUNCTION get_token_supply(t_id bigint)
                RETURNS TABLE(non_circulating bigint, liquid bigint, locked bigint)
                LANGUAGE plpgsql STABLE COST 200000 PARALLEL SAFE
                AS $$
                BEGIN
                RETURN QUERY
                with non_circ as (
                    select id from public.addresses a where address in (
                        '9hXmgvzndtakdSAgJ92fQ8ZjuKirWAw8tyDuyJrXP6sKHVpCz8XbMANK3BVJ1k3WD6ovQKTCasjKL5WMncRB6V9HvmMnJ2WbxYYjtLFS9sifDNXJWugrNEgoVK887bR5oaLZA95yGkMeXVfanxpNDZYaXH9KpHCpC5ohDtaW1PF17b27559toGVCeCUNti7LXyXV8fWS1mVRuz2PhLq5mB7hg2bqn7CZtVM8ntbUJpjkHUc9cP1R8Gvbo1GqcNWgM7gZkr2Dp514BrFz1cXMkv7TYEqH3cdxX9c82hH6fdaf3n6avdtZ5bgqerUZVDDW6ZsqxrqTyTMQUUirRAi3odmMGmuMqDJbU3Z1VnCF9NBow7jrKUDSgckDZakFZNChsr5Kq1kQyNitYJUh9fra1jLHCQ9yekz3te9E'
                    ,'HNLdwoHRsUSevguzRajzvy1DLAvUJ9YgQezQq6GGZiY4TmU9VDs2ae8mRpQkfEnLmuUKyJibZD2bXR2yoo1p8T5WCRKPn4rJVJ2VR2LvRBk8ViCmhcume5ubWaySXTUqpftEaaURTM6KSFxe4QbRFbToyPzZ3JJmjoDn4WzHh5ioXZMj7AX6xTwJvFmzPuko9BqDk5z1RJtD1wP4kd8sSsLN9P2YNQxmUGDEBYHaDCoAhY7Pg5oKit6ZyqMynoiycWqctfg1EHhMUKCTJsZNnidU961ri98RaYP4CfEwYQ3d9dRVuC6S1n7J1wPPHYqmUBgJCGWbTULayXUowSSmRuZUkQYGo9vvNaEpB7ManiLsX1n8cBYwN4XoVsY24mCfptBP86P4rZ5fgcr9mYtQ9nG934DMDZBbjs81VzCupB6KVrGCe1WtYSr6c1DwkNAinBMwqcqxTznXZUvfBsjDSCtJzCut44xcc7Zsy9mWz2B2pqhdKsX83BVzMDDM5hnjXTShYfauJGs81'
                    ,'MUbV38YgqHy7XbsoXWF5z7EZm524Ybdwe5p9WDrbhruZRtehkRPT92imXer2eTkjwPDfboa1pR3zb3deVKVq3H7Xt98qcTqLuSBSbHb7izzo5jphEpcnqyKJ2xhmpNPVvmtbdJNdvdopPrHHDBbAGGeW7XYTQwEeoRfosXzcDtiGgw97b2aqjTsNFmZk7khBEQywjYfmoDc9nUCJMZ3vbSspnYo3LarLe55mh2Np8MNJqUN9APA6XkhZCrTTDRZb1B4krgFY1sVMswg2ceqguZRvC9pqt3tUUxmSnB24N6dowfVJKhLXwHPbrkHViBv1AKAJTmEaQW2DN1fRmD9ypXxZk8GXmYtxTtrj3BiunQ4qzUCu1eGzxSREjpkFSi2ATLSSDqUwxtRz639sHM6Lav4axoJNPCHbY8pvuBKUxgnGRex8LEGM8DeEJwaJCaoy8dBw9Lz49nq5mSsXLeoC4xpTUmp47Bh7GAZtwkaNreCu74m9rcZ8Di4w1cmdsiK1NWuDh9pJ2Bv7u3EfcurHFVqCkT3P86JUbKnXeNxCypfrWsFuYNKYqmjsix82g9vWcGMmAcu5nagxD4iET86iE2tMMfZZ5vqZNvntQswJyQqv2Wc6MTh4jQx1q2qJZCQe4QdEK63meTGbZNNKMctHQbp3gRkZYNrBtxQyVtNLR8xEY8zGp85GeQKbb37vqLXxRpGiigAdMe3XZA4hhYPmAAU5hpSMYaRAjtvvMT3bNiHRACGrfjvSsEG9G2zY5in2YWz5X9zXQLGTYRsQ4uNFkYoQRCBdjNxGv6R58Xq74zCgt19TxYZ87gPWxkXpWwTaHogG1eps8WXt8QzwJ9rVx6Vu9a5GjtcGsQxHovWmYixgBU8X9fPNJ9UQhYyAWbjtRSuVBtDAmoV1gCBEPwnYVP5GCGhCocbwoYhZkZjFZy6ws4uxVLid3FxuvhWvQrVEDYp7WRvGXbNdCbcSXnbeTrPMey1WPaXX'
                    )
                )
                select
                sum(case when addr_type = 0 then token_amount else 0 end)::bigint as non_circulating_supply,
                sum(case when addr_type = 1 then token_amount else 0 end)::bigint as liquid_supply,
                sum(case when addr_type = 2 then token_amount else 0 end)::bigint as locked_supply
                from (
                select aib.*,
                (case when aib.address = any(select id from non_circ) then 0
                when length(a.address) = 51 and starts_with(a.address, '9') then 1
                else 2
                end) as addr_type
                from public.asset_in_box aib
                join public.addresses a on a.id = aib.address
                where aib.token_id = t_id
                and aib.spent is null
                ) as main;

                END;
                $$
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS get_token_supply")
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS get_wrapped_tokens")
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP TYPE IF EXISTS token")
            .await?;
        Ok(())
    }
}
