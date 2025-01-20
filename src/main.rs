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
    let pair_level: Vec<&str> = parts[1].split(":").collect();

    let mut pair = pair_level[0].to_uppercase();
    let level: u8 = pair_level[1].parse().unwrap();
    let exchange = parts[0].to_uppercase();

    match exchange.as_str() {
        "BINANCE" => {
            pair = pair.replace("_", "");
            exchange_binance::subscribe_to_pair(&pair, &level).await;
        }
        "BYBIT" => {
            pair = pair.replace("_", "");
            exchange_bybit::subscribe_to_pair(&pair, &level).await;
        }
        "KRAKEN" => {
            pair = pair.replace("_", "/");
            exchange_kraken::subscribe_to_pair(&pair, &level).await;
        }
        "POLONIEX" => {
            exchange_poloniex::subscribe_to_pair(&pair, &level).await;
        }
        _ => {
            println!("unknown exhange: {}", parts[0])
        }
    }
}
