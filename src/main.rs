use futures_util::StreamExt;
use std::env;

mod exchange_binance;
mod exchange_bybit;
mod exchange_kraken;
mod exchange_poloniex;

#[tokio::main]
async fn main() {
    // parameters
    let args: Vec<String> = env::args().collect();
    let parts: Vec<&str> = args[1].split("__").collect();
    let exchange = parts[0].to_lowercase();

    match exchange.as_str() {
        "binance" => {
            let pair = parts[1].to_uppercase().replace("_", "");
            exchange_binance::subscribe_to_pair(&pair).await;
        }
        "bybit" => {
            let pair = parts[1].to_uppercase().replace("_", "");
            exchange_bybit::subscribe_to_pair(&pair).await;
        }
        "kraken" => {
            let pair = parts[1].to_uppercase().replace("_", "/");
            exchange_kraken::subscribe_to_pair(&pair).await;
        }
        "poloniex" => {
            let pair = parts[1].to_uppercase();
            exchange_poloniex::subscribe_to_pair(&pair).await;
        }
        _ => {
            println!("unknown exhange: {}", parts[0])
        }
    }
}
