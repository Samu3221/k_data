# k_data, KALSHI raw data collection API for POSTGRESQL WRITTEN IN RUST

> The project enables users to gather and store kalshi orderbook, public trades and market ticker updates in a postgresql database. 

##  Features

* Fast data and accurate data collection 
* Kalshi Orderbook, public trades and market ticker, data collection (More to come)
* Tables used for the data collection ()

## QUICK START 

Run these commands to get started with the api:

### Installation 
```sh
# Download the the crate
cargo add k_data

# navigate to your database
psql 'dbname'
```

Add the following tables:
```sql
CREATE TABLE orderbook_delta (
    id SERIAL PRIMARY KEY NOT NULL,
    market_ticker VARCHAR(26) NOT NULL,
    price_dollars VARCHAR(20) NOT NULL,
    delta_fp VARCHAR(30) NOT NULL,
    side VARCHAR(10) NOT NULL,
    ts TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE orderbook_snapshot (
    id SERIAL PRIMARY KEY NOT NULL,
    ts TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    market_ticker VARCHAR(26) NOT NULL,
    yes_dollars_fp JSON,
    no_dollars_fp JSON
);

CREATE TABLE trades (
    id Serial PRIMARY KEY NOT NULL,
    market_ticker VARCHAR(26) NOT NULL,
    yes_price_dollars VARCHAR(20) NOT NULL,
    no_price_dollars VARCHAR(20) NOT NULL,
    count_fp VARCHAR(30) NOT NULL,
    taker_side VARCHAR(10) NOT NULL,
    taker_outcome_side VARCHAR(10) NOT NULL,
    taker_book_side VARCHAR(10) NOT NULL,
    ts_ms TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE market_ticker (
    id Serial PRIMARY KEY NOT NULL,
    market_ticker VARCHAR(26) NOT NULL,
    price_dollars VARCHAR(20) NOT NULL,
    yes_bid_price_dollars VARCHAR(20) NOT NULL,
    yes_ask_price_dollars VARCHAR(20) NOT NULL,
    volume_fp VARCHAR(20) NOT NULL,
    ts_ms TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

You should now be able to follow the usage examples without problem


## Usage examples

For getting the url please use this format:
> postgresql://[user[:password]@][netloc][:port][/dbname][?param1=value1&...]


### 15 min ticker parser
```rust
use k_data::operations::ticker_functions::functions::get_up_down_ticker; // for getting the updown ticker 

fn main() {
    let market_ticker: String = get_up_down_ticker("KXBTC15M"); // insert the series 
    println!("{market_ticker}"); // returns the current active ticker for a updown market
}

```

### Gathering data for a 15 min up down market

```rust
use k_data::operations::data_functions::kalshi_data::gather_data;  // function to 
use k_data::operations::ticker_functions::functions::get_up_down_ticker; // for getting the updown ticker 

#[tokio::main]
async fn start_gathering_data() {
    let market_ticker: String = get_up_down_ticker("KXBTC15M"); // insert the series // in this case the  BTC 15M up down market
    let url: String = format!("postgresql://postgres@localhost:5432/{}", "kalshi_data"); // database is named 'kalshi_data'
    gather_data(&market_ticker, url,"secret_key").await; // begins uploading the data to the DATABASE /
}

start_gathering_data().await; 
```


## Modifications

Until the necessary features for the api are added, we heavily incentivize modifications to the crate.

## FUTURE PROSPECTS 

For the future of this project it intends to: expand on the amount of supported websocket channels it supports.

* Expand on the amount of supported websocket channels it supports.
* Demo functionality
* Create  more customization for postgres table entries
* Create tables function

## Please refer to the KALSHI API for any questions regarding JSON structures
Here: [Kalshi api docs](https://docs.kalshi.com/welcome).

## License

This project is licensed under the [MIT License](LICENSE).