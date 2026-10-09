use crate::operations::data_functions::kalshi_data::gather_data;  // function to 
use crate::operations::ticker_functions::functions::get_up_down_ticker; // for getting the updown ticker 

#[tokio::main]
pub async fn start_gathering_data() {
    let market_ticker: String = get_up_down_ticker("KXBTC15M"); // insert the series 
    let url: String = format!("postgresql://postgres@localhost:5432/{}", "kalshi_data"); // database is named 'kalshi_data'
    gather_data(&market_ticker, url,"secret_key").await; // begins uploading the data to the DATABASE /
}

