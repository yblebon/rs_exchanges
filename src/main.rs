use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use serde_json::Value;
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

mod exchange_binance;
mod exchange_bybit;
mod exchange_kraken;
mod exchange_poloniex;

#[tokio::main]
async fn main() {
    // parameters
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "binance" => {
            let pair = args[2].replace("_", "");
            exchange_binance::subscribe_to_pair(&pair).await;
        }
        "bybit" => {
            exchange_bybit::subscribe_to_pair(&args[2]).await;
        }
        "kraken" => {
            exchange_kraken::subscribe_to_pair(&args[2]).await;
        }
        "poloniex" => {
            exchange_poloniex::subscribe_to_pair(&args[2]).await;
        }
        _ => {
            println!("unknown exhange: {}", args[1])
        }
    }
}
