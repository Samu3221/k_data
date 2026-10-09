use k_data::auth::authend::authentication::obtain_api_key;
use k_data::operations::data_functions::kalshi_data::gather_data; // function to
use k_data::operations::ticker_functions::functions::get_up_down_ticker; // for getting the updown ticker 

#[tokio::main]
pub async fn main() {
    let market_ticker: String = get_up_down_ticker("KXBTC15M"); // insert the series 
    let db_url: String = format!("postgresql://postgres@localhost:5432/{}", "kalshi_data"); // database is named 'kalshi_data'
    let api_key: String = obtain_api_key("KALSHI_API_KEY");
    gather_data(&market_ticker, db_url, &api_key, "prod2.txt").await;
}
