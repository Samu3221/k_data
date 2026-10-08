use serde_json; // parsing json
use sqlx; // connecting to postgresql
use std::error::Error;

pub mod functions {

    use super::*;

    pub async fn insert_into_trades(
        values: serde_json::Value,
        database_pool: &sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO trades (market_ticker, yes_price_dollars, no_price_dollars, count_fp, taker_side, taker_outcome_side, taker_book_side, ts_ms) VALUES ($1 ,$2, $3, $4, $5, $6, $7, $8)";

        // CREATE TABLE trades (
        //     id Serial PRIMARY KEY NOT NULL,
        //     market_ticker VARCHAR(26) NOT NULL,
        //     yes_price_dollars VARCHAR(6) NOT NULL,
        //     no_price_dollars VARCHAR(6) NOT NULL,
        //     count_fp VARCHAR(15) NOT NULL,
        //     taker_side VARCHAR(2) NOT NULL,
        //     taker_outcome_side VARCHAR(2) NOT NULL,
        //     taker_book_side VARCHAR(3) NOT NULL,
        //     ts_ms BIGINT
        // );

        sqlx::query(query)
            .bind(&values["market_ticker"])
            .bind(&values["yes_price_dollars"])
            .bind(&values["no_price_dollars"])
            .bind(&values["count_fp"])
            .bind(&values["taker_side"])
            .bind(&values["taker_outcome_side"])
            .bind(&values["taker_book_side"])
            .bind(&values["ts_ms"])
            .execute(database_pool)
            .await?;

        Ok(())
    }

    pub async fn insert_into_orderbook_delta(
        values: serde_json::Value,
        database_pool: &sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO trades (market_ticker, price_dollars, delta_fp, side, ts) VALUES ($1 ,$2, $3, $4, $5)";

        // CREATE TABLE orderbook_delta (
        //     id SERIAL PRIMARY KEY NOT NULL,
        //     market_ticker VARCHAR(26) NOT NULL,
        //     price_dollars VARCHAR(6) NOT NULL,
        //     delta_fp VARCHAR(15) NOT NULL,
        //     side VARCHAR(3) NOT NULL,
        //     ts TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        // );

        sqlx::query(query)
            .bind(&values["market_ticker"])
            .bind(&values["price_dollars"])
            .bind(&values["delta_fp"])
            .bind(&values["side"])
            .bind(&values["ts"])
            .execute(database_pool)
            .await?;

        Ok(())
    }

    pub async fn insert_into_market_ticker(
        values: serde_json::Value,
        database_pool: &sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO trades (market_ticker, price_dollars, yes_price_dollars, no_price_dollars, volume_fp, ts_ms) VALUES ($1 ,$2, $3, $4, $5, $6)";

        // CREATE TABLE market_ticker (
        //     id Serial PRIMARY KEY NOT NULL,
        //     market_ticker VARCHAR(26) NOT NULL,
        //     price_dollars VARCHAR(6) NOT NULL,
        //     yes_price_dollars VARCHAR(6) NOT NULL,
        //     no_price_dollars VARCHAR(6) NOT NULL,
        //     volume_fp VARCHAR(20) NOT NULL,
        //     ts_ms TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        // );

        sqlx::query(query)
            .bind(&values["market_ticker"])
            .bind(&values["ts_ms"])
            .bind(&values["price_dollars"])
            .bind(&values["yes_price_dollars"])
            .bind(&values["no_price_dollars"])
            .bind(&values["volume_fp"])
            .execute(database_pool)
            .await?;

        Ok(())
    }

    pub async fn insert_into_orderbook_snapshot(
        values: serde_json::Value,
        database_pool: &sqlx::PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let query =
            "INSERT INTO trades (market_ticker, yes_dollars_fp, no_dollars_fp) VALUES ($1 ,$2, $3)";

        // CREATE TABLE orderbook_snapshot (
        //     id SERIAL PRIMARY KEY NOT NULL,
        //     ts TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        //     market_ticker VARCHAR(26) NOT NULL,
        //     yes_dollars_fp JSON,
        //     no_dollars_fp JSON
        // );

        sqlx::query(query)
            .bind(&values["market_ticker"])
            .bind(&values["yes_price_dollars"])
            .bind(&values["no_price_dollars"])
            .execute(database_pool)
            .await?;

        Ok(())
    }
}
