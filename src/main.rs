use std::env; // for getting the api and secret key respectively

use futures_util::{SinkExt, StreamExt}; // sending and receiving messages

use serde_json; // parsing json
use serde_json::json; // creating json objects

use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::Message}; // for websockets and websocket messages // for request

use crate::websockets::get_data; // websocket connections

use k_data::auth::authend::authentication::sign; // for signing and authentication of messages
use k_data::operations::sql_functions::functions::{
    insert_into_market_ticker, insert_into_orderbook_delta, insert_into_orderbook_snapshot,
    insert_into_trades,
};
use k_data::operations::ticker_functions::functions::get_up_down_ticker; // for getting the updown ticker 

use chrono; // for adding timestamp to requests and adding auto adding the right ticker

use sqlx::postgres;

static URL: &str = "wss://external-api-ws.kalshi.com/trade-api/ws/v2"; // the url

#[tokio::main]
async fn main() {
    let market_ticker: String = get_up_down_ticker("KXBTC15M");
    let url: String = format!("postgresql://postgres@localhost:5432/{}", "kalshi");
    get_data(&market_ticker, url).await;
}

pub mod websockets {
    // web socket module

    use super::*; // imports all modules above 

    fn obtain_api_key() -> String {
        // function for obtaining the api keys get called every time to not let them persist in memory
        dotenv::dotenv().ok();
        let api_key: String = match env::var("KALSHI_API_KEY") {
            Ok(key) => key,
            Err(e) => panic!("Program was unable to obtain API key from .env!!!: {e}"),
        };

        api_key // returning the api key
    }

    // make web socket struct where we run the operation out of wif the struct is dropped then we end connection and start a new one
    pub async fn get_data(market_ticker: &str, database_url: String) {
        let db_conn: sqlx::Pool<sqlx::Postgres> = postgres::PgPool::connect(&database_url).await.unwrap();

        let api_key = obtain_api_key();

        let timestamp = chrono::DateTime::timestamp_millis(&chrono::Utc::now());

        let signature = sign("prod2.txt", timestamp as u64, "GET", "/trade-api/ws/v2");

        let mut request = URL.into_client_request().unwrap();
        request
            .headers_mut()
            .insert("KALSHI-ACCESS-KEY", format!("{}", api_key).parse().unwrap());
        request.headers_mut().insert(
            "KALSHI-ACCESS-SIGNATURE",
            format!("{}", signature).parse().unwrap(),
        );
        request.headers_mut().insert(
            "KALSHI-ACCESS-TIMESTAMP",
            format!("{}", timestamp).parse().unwrap(),
        );

        // starting the websockets
        let (mut ws, _) = match connect_async(request).await {
            Ok(websockets) => websockets,
            Err(e) => {
                panic!("Something went wrong trying to establish a websocket connection: {e:?}")
            }
        };

        let subscription_msg = json!(
            {
                "id": 1,
                "cmd": "subscribe",
                "params": {
                    "channels": ["orderbook_delta"],
                    "market_ticker": market_ticker,
                },
            }
        );

        match ws
            .send(Message::Text(subscription_msg.to_string().into()))
            .await
        {
            // sending the auth message
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        }

        let subscription_msg = json!(
            {
                "id": 1,
                "cmd": "subscribe",
                "params": {
                    "channels": ["ticker"],
                    "market_ticker": market_ticker
                },
            }
        );

        match ws
            .send(Message::Text(subscription_msg.to_string().into()))
            .await
        {
            // sending the auth message
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        }

        let subscription_msg = json!(
            {
                "id": 2,
                "cmd": "subscribe",
                "params": {
                    "channels": ["trade"],
                    "market_ticker": market_ticker
                }
            }
        );

        match ws
            .send(Message::Text(subscription_msg.to_string().into()))
            .await
        {
            // sending the auth message
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        }

        // here we write all new prices that are not the same as those already placed
        // !!! be very careful of null values need to get that handled
        // will get handled when  we introduce match

        while let Some(Ok(message)) = ws.next().await {
            if let tokio_tungstenite::tungstenite::Message::Text(message_string) = message {
                let parsed_message: serde_json::Value = match serde_json::from_str(&message_string)
                {
                    Ok(msg) => msg,
                    Err(_) => panic!("Panicked because parsing of incoming message failed"), // panics because that kills the thread locally and try again
                };
                let message_type: &serde_json::Value = &parsed_message["type"];
                let parsed_msg = &parsed_message["msg"];
                match message_type.as_str() {
                    Some("trade") => insert_into_trades(parsed_msg, &db_conn).await.unwrap(),
                    Some("ticker") => insert_into_market_ticker(parsed_msg, &db_conn)
                        .await
                        .unwrap(),
                    Some("orderbook_delta") => insert_into_orderbook_delta(parsed_msg, &db_conn)
                        .await
                        .unwrap(),
                    Some("orderbook_snapshot") => {
                        insert_into_orderbook_snapshot(parsed_msg, &db_conn)
                            .await
                            .unwrap()
                    }
                    _ => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::main]
    #[test]
    async fn testy() {
        let market_ticker: String = get_up_down_ticker("KXBTC15M"); // insert the series 
        let url: String = format!("postgresql://postgres@localhost:5432/{}", "kalshi");
        websockets::get_data(&market_ticker, url).await;
    }
}
