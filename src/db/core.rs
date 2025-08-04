use anyhow::{Context, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};

use super::models::CandleSnapshot;

pub async fn init_postgres_pool(url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .context("failed connect database")?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn insert_candle_batches(pool: &PgPool, candles: CandleSnapshot) -> Result<()> {
    let symbol = candles.symbol;
    let interval = candles.interval;
    let vec_open_time: Vec<i64> = candles
        .datas
        .iter()
        .map(|candle| candle.open_time)
        .collect();
    let vec_close_time: Vec<i64> = candles
        .datas
        .iter()
        .map(|candle| candle.close_time)
        .collect();
    let vec_open_price: Vec<f64> = candles
        .datas
        .iter()
        .map(|candle| candle.open_price.parse().unwrap())
        .collect();
    let vec_close_price: Vec<f64> = candles
        .datas
        .iter()
        .map(|candle| candle.close_price.parse().unwrap())
        .collect();
    let vec_low_price: Vec<f64> = candles
        .datas
        .iter()
        .map(|candle| candle.low_price.parse().unwrap())
        .collect();
    let vec_volume: Vec<f64> = candles
        .datas
        .iter()
        .map(|candle| candle.volume.parse().unwrap())
        .collect();
    let vec_trades: Vec<i64> = candles
        .datas
        .iter()
        .map(|candle| candle.number_of_trades)
        .collect();

    sqlx::query!(
            "
                INSERT INTO hyper_candle_snapshot (symbol, interval, open_time, close_time, open_price, close_price, low_price, volume, number_of_trades)
                SELECT
                    $1::varchar,
                    $2::varchar,
                    unnested_data.open_time,
                    unnested_data.close_time,
                    unnested_data.open_price,
                    unnested_data.close_price,
                    unnested_data.low_price,
                    unnested_data.volume,
                    unnested_data.number_of_trades
                FROM UNNEST(
                    $3::int8[],
                    $4::int8[],
                    $5::float8[],
                    $6::float8[],
                    $7::float8[],
                    $8::float8[],
                    $9::int8[]
                ) AS unnested_data(open_time, close_time, open_price, close_price, low_price, volume, number_of_trades)
            ", &symbol, &interval, &vec_open_time[..], &vec_close_time[..], &vec_open_price[..], &vec_close_price[..], &vec_low_price[..], &vec_volume[..], &vec_trades[..])
            .execute(pool).await?;

    Ok(())
}

pub async fn get_last_close_time(
    pool: &PgPool,
    symbol: &str,
    interval: &str,
) -> Result<Option<i64>> {
    match  sqlx::query!(
        "SELECT close_time FROM hyper_candle_snapshot WHERE symbol = $1::varchar AND interval = $2::varchar ORDER BY close_time DESC LIMIT 1"
        , symbol, interval)
        .fetch_optional(pool).await? {
        Some(record) => {
           Ok(Some(record.close_time))
        },
        None => Ok(None),
    }
}
