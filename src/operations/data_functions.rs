use std::env; // for getting the api and secret key respectively

use futures_util::{SinkExt, StreamExt}; // sending and receiving messages

use serde_json; // parsing json
use serde_json::json; // creating json objects

use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::Message}; // for websockets and websocket messages // for request

use crate::auth::authend::authentication::sign; // for signing and authentication of messages
use crate::operations::sql_functions::functions::{
    insert_into_market_ticker, insert_into_orderbook_delta, insert_into_orderbook_snapshot,
    insert_into_trades,
};

use chrono; // for adding timestamp to requests and adding auto adding the right ticker

use sqlx::postgres;

static URL: &str = "wss://external-api-ws.kalshi.com/trade-api/ws/v2"; // the url

pub mod kalshi_data {
    // web socket module

    use super::*; // imports all modules above 

    // make web socket struct where we run the operation out of wif the struct is dropped then we end connection and start a new one
    pub async fn gather_data(
        market_ticker: &str,
        database_url: String,
        api_key: &str,
        secret_key_path: &str,
    ) {
        let db_conn: sqlx::Pool<sqlx::Postgres> =
            match postgres::PgPool::connect(&database_url).await {
                Ok(connection) => connection,
                Err(error) => panic!(" ¨program failed to connect to database: {}", error),
            };

        let timestamp = chrono::DateTime::timestamp_millis(&chrono::Utc::now());

        let signature = sign(secret_key_path, timestamp as u64, "GET", "/trade-api/ws/v2");

        let mut request = match URL.into_client_request() {
            Ok(request) => request,
            Err(error) => panic!(
                "The program failed turning the request url into a client request: {}",
                error
            ),
        };
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
                    Some("trade") => match insert_into_trades(parsed_msg, &db_conn).await {
                        Ok(_) => (),
                        Err(error) => {
                            panic!("The Program failed to insert into trades TABLE: {}", error)
                        }
                    },
                    Some("ticker") => match insert_into_market_ticker(parsed_msg, &db_conn).await {
                        Ok(_) => (),
                        Err(error) => panic!(
                            "The Program failed to insert into market_ticker TABLE: {}",
                            error
                        ),
                    },
                    Some("orderbook_delta") => {
                        match insert_into_orderbook_delta(parsed_msg, &db_conn).await {
                            Ok(_) => (),
                            Err(error) => panic!(
                                "The Program failed to insert into orderbook_delta TABLE: {}",
                                error
                            ),
                        }
                    }
                    Some("orderbook_snapshot") => {
                        match insert_into_orderbook_snapshot(parsed_msg, &db_conn).await {
                            Ok(_) => (),
                            Err(error) => panic!(
                                "The Program failed to insert into orderbook_snapshot TABLE: {}",
                                error
                            ),
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::authend::authentication::obtain_api_key;
    use crate::operations::ticker_functions::functions::get_up_down_ticker; // for getting the updown ticker

    use super::*;
    #[tokio::main]
    #[test]
    async fn testy() {
        let market_ticker: String = get_up_down_ticker("KXBTC15M"); // insert the series 
        let url: String = format!("postgresql://postgres@localhost:5432/{}", "kalshi");
        let api_key: String = obtain_api_key("KALSHI_API_KEY");
        kalshi_data::gather_data(&market_ticker, url, &api_key, "prod2.txt").await;
    }
}
