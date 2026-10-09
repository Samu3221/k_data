# k_data, KALSHI raw data collection API for POSTGRESQL WRITTEN IN RUST

> The project enables users to gather and store kalshi orderbook, public trades and market ticker updates in a postgresql database. 
> The tool serves as a data collection tool for the desired ticker, and also includes an automatic parser of 15 minute up down market tickers.
> includes a tables used for the data collection


## Installation 
```sh
cargo add k_data
```

## Usage examples

### 15 min ticker parser
```rust
use k_data::;

let market_ticker: String = get_up_down_ticker("KXBTC15M"); // insert the series 
// returns the right ticker name to be used for the ticker 
```

### Gathering data for a 15 min up down market

```rust
let market_ticker: String = get_up_down_ticker("KXBTC15M"); // insert the series 
let url: String = format("postgresql://postgres@localhost:5432/{}", "kalshi"); // database 
websockets::get_data(&market_ticker, url).await;
```


# Modifications

Until the necessary features for the api are added, we heavily incentivize modifications to the crate.

# FUTURE PROSPECTS 

For the future of this project it intends to: expand on the amount of supported websocket channels it supports.

* Expand on the amount of supported websocket channels it supports.

* Demo functionality

* Create  more customization for postgres table entries

## License

This project is licensed under the [MIT License](LICENSE).