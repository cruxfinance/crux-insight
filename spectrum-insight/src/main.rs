use anyhow::Result;
use sqlx::{Executor, PgPool};

async fn init_db() -> Result<()> {
    let pool = PgPool::connect("postgres://postgres:test@localhost/lagerg").await?;
    let res = pool.execute(
        r#"
    CREATE SCHEMA IF NOT EXISTS spectrum AUTHORIZATION postgres;
    
    CREATE TABLE IF NOT EXISTS spectrum.spectrum_boxes
    (
        box_id character varying(64) NOT NULL,
        value bigint,
        "tokenName" character varying(100),
        "poolId" character varying(64),
        "tokenAmount" bigint,
        "tokenDecimals" smallint,
        "timestamp" timestamp without time zone,
        global_index bigint,
        PRIMARY KEY (box_id)
    );

    ALTER TABLE IF EXISTS spectrum.spectrum_boxes
        OWNER to postgres;

    CREATE OR REPLACE FUNCTION spectrum.getohlcv(
        pool_id character varying,
        time_resolution interval,
        from_time timestamp with time zone,
        to_time timestamp with time zone,
        flipped boolean DEFAULT false)
        RETURNS TABLE(open double precision, high double precision, low double precision, close double precision, volume double precision, "time" integer) 
        LANGUAGE 'sql'
        COST 100
        VOLATILE PARALLEL UNSAFE
        ROWS 1000

    AS $BODY$
    
    with time_table as (
		select to_timestamp(
                floor(EXTRACT(epoch FROM dt) / EXTRACT(epoch FROM time_resolution))
                * EXTRACT(epoch FROM time_resolution)) as dd from generate_series
            ( from_time 
            , to_time
            , time_resolution) dt
		), cte as (
        SELECT 
            CAST("value" AS DOUBLE PRECISION)/1000000000 as "value", 
            LAG(CAST("value" AS DOUBLE PRECISION)/1000000000,1) OVER(ORDER BY "timestamp", "global_index") AS "previousValue", 
            CAST("tokenAmount" AS DOUBLE PRECISION)/(10^"tokenDecimals") as "tokenAmount", 
            LAG(CAST("tokenAmount" AS DOUBLE PRECISION)/(10^"tokenDecimals"),1) OVER(ORDER BY "timestamp", "global_index") AS "previousTokenAmount", 
            to_timestamp(
                floor(EXTRACT(epoch FROM "timestamp") / EXTRACT(epoch FROM time_resolution))
                * EXTRACT(epoch FROM time_resolution)) as "time", 
            "global_index"
        FROM spectrum.spectrum_boxes
        where "poolId" = pool_id
        and "timestamp" >= (select min(dd) from time_table)
        and "timestamp" <= (select max(dd) + time_resolution from time_table)
    ) ,
    cte2 as (select first_value(case when flipped then "previousValue"/"previousTokenAmount" else "previousTokenAmount"/"previousValue" end) over (partition by "time" order by "global_index") as "open", 
        max(case when flipped then greatest("value"/"tokenAmount","previousValue"/"previousTokenAmount") else greatest("tokenAmount"/"value","previousTokenAmount"/"previousValue") end) over (partition by "time") as "high", 
        min(case when flipped then least("value"/"tokenAmount","previousValue"/"previousTokenAmount") else least("tokenAmount"/"value","previousTokenAmount"/"previousValue") end) over (partition by "time") as "low", 
        first_value(case when flipped then "value"/"tokenAmount" else "tokenAmount"/"value" end) over (partition by "time" order by "global_index" desc) as "close", 
        sum(abs("previousValue"-"value")) over (partition by "time") as "volume",
        "time"
            from cte
    where (("tokenAmount" > "previousTokenAmount" and "value" < "previousValue") or
            ("tokenAmount" < "previousTokenAmount" and "value" > "previousValue"))
    ),
	cte3 as (
    select 
		max("open") as "open", 
		max("high") as "high", 
		max("low") as "low", 
		max("close") as "close", 
		max("volume") as "volume", 
		floor(extract(epoch from ts.dd)) as "time"
    from time_table as ts left join cte2 on ts.dd = cte2."time"
    group by ts.dd)
	select 
		coalesce("open", first_value("close") over (partition by value_partition order by "time")),
		coalesce("high", first_value("close") over (partition by value_partition order by "time")),
		coalesce("low", first_value("close") over (partition by value_partition order by "time")),
		first_value("close") over (partition by value_partition order by "time"),
		coalesce("volume",0),
		"time" from (
			select *, 
			sum(case when "close" is null then 0 else 1 end) over (order by "time") as value_partition
			from cte3
		) as final_sub
	order by "time";

    $BODY$;

    ALTER FUNCTION spectrum.getohlcv(character varying, interval, timestamp with time zone, timestamp with time zone, boolean)
    OWNER TO postgres;

    CREATE MATERIALIZED VIEW IF NOT EXISTS spectrum.pool_tvl_mat
    TABLESPACE pg_default
    AS
    WITH cte AS (
            SELECT spectrum_boxes."poolId",
                first_value(spectrum_boxes.value) OVER (PARTITION BY spectrum_boxes."poolId" ORDER BY spectrum_boxes.global_index DESC) AS value,
                spectrum_boxes."tokenName"
            FROM spectrum.spectrum_boxes
            )
    SELECT cte."poolId",
        max(cte.value) AS value,
        cte."tokenName"
    FROM cte
    GROUP BY cte."poolId", cte."tokenName"
    WITH DATA;

    ALTER TABLE IF EXISTS spectrum.pool_tvl_mat
        OWNER TO postgres;"#
    ).await;
    println!("{:?}", res);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = init_db().await;
    Ok(())
}

// SELECT box_id, data->'value' as "value", tok.token_name as "tokenName", data->'assets'->0->>'tokenId' as "poolId", (data->'assets'->2->'amount') as "tokenAmount", tok.decimals as "tokenDecimals", blok.timestamp, b.id
// 	FROM public.boxes b
// 	join public.addresses addr on b.address = addr.id
// 	join public.tokens tok on tok.token_id = (data->'assets'->2->>'tokenId')::text
// 	join public.blocks blok on blok.height = b.created
// 	where addr.ergotree = '1999030f0400040204020404040405feffffffffffffffff0105feffffffffffffffff01050004d00f040004000406050005000580dac409d819d601b2a5730000d602e4c6a70404d603db63087201d604db6308a7d605b27203730100d606b27204730200d607b27203730300d608b27204730400d6099973058c720602d60a999973068c7205027209d60bc17201d60cc1a7d60d99720b720cd60e91720d7307d60f8c720802d6107e720f06d6117e720d06d612998c720702720fd6137e720c06d6147308d6157e721206d6167e720a06d6177e720906d6189c72117217d6199c72157217d1ededededededed93c27201c2a793e4c672010404720293b27203730900b27204730a00938c7205018c720601938c7207018c72080193b17203730b9593720a730c95720e929c9c721072117e7202069c7ef07212069a9c72137e7214067e9c720d7e72020506929c9c721372157e7202069c7ef0720d069a9c72107e7214067e9c72127e7202050695ed720e917212730d907216a19d721872139d72197210ed9272189c721672139272199c7216721091720b730e'
// 	LIMIT 100;
