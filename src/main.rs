use std::env; // for getting the api and secret key respectively

use futures_util::{SinkExt, StreamExt}; // sending and receiving messages

use serde_json; // parsing json
use serde_json::json; // creating json objects

use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::Message}; // for websockets and websocket messages // for request

use crate::websockets::run_sockets; // websocket connections

use k_data::auth::authend::authentication::sign; // for signing and authentication of messages

use chrono; // for adding timestamp to requests and adding auto adding the right ticker

static URL: &str = "wss://external-api-ws.kalshi.com/trade-api/ws/v2"; // the url

#[tokio::main]
async fn main() {
    run_sockets("KXBTC15M-26OCT050830-30").await;
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
    pub async fn run_sockets(market_ticker: &str) {
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

        // let subscription_msg = json!(
        //     {
        //         "id": 1,
        //         "cmd": "subscribe",
        //         "params": {
        //             "channels": ["orderbook_delta"],
        //             "market_ticker": market_ticker,
        //         },
        //     }
        // );
        // println!("{}",subscription_msg);
        // match ws
        //     .send(Message::Text(subscription_msg.to_string().into()))
        //     .await
        // {
        //     // sending the auth message
        //     Ok(_) => (),
        //     Err(e) => panic!("{e}"),
        // }

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

        // let subscription_msg = json!(
        //     {
        //         "id": 2,
        //         "cmd": "subscribe",
        //         "params": {
        //             "channels": ["trade"],
        //             "market_ticker": market_ticker
        //         }
        //     }
        // );
        // match ws
        //     .send(Message::Text(subscription_msg.to_string().into()))
        //     .await
        // {
        //     // sending the auth message
        //     Ok(_) => (),
        //     Err(e) => panic!("{e}"),
        // }

        // here we write all new prices that are not the same as those already placed
        // !!! be very careful of null values need to get that handled
        // will get handled when  we introduce match
        while let Some(Ok(message)) = ws.next().await {
            println!("{}", message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::main]
    #[test]
    async fn testy() {
        websockets::run_sockets("KXBTC15M-26OCT050915-15").await;
    }
}
