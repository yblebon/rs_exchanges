use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use serde_json::Value;
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

mod exchange_binance;
mod exchange_bybit;

#[tokio::main]
async fn main() {
    // parameters
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "binance" => {
            exchange_binance::subscribe_to_pair(&args[2]).await;
        }
        "bybit" => {
            exchange_bybit::subscribe_to_pair(&args[2]).await;
        }
        _ => {
            println!("unknown exhange: {}", args[1])
        }
    }
}
