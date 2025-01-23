use futures::future::join_all;
use futures_util::StreamExt;
use serde_json::Value;
use std::env;

mod exchange;
use exchange::binance;
use exchange::bybit;
use exchange::kraken;
use exchange::poloniex;

mod event;

use crate::event::Ticker;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    feed: Vec<String>,
}

pub async fn subscribe_to_pair<F>(exchange: String, pair: String, level: u8, cb: F)
where
    F: Fn(Value),
{
    match exchange.as_str() {
        "BINANCE" => {
            let p = pair.replace("_", "");
            binance::subscribe_to_pair(p, level, print_msg).await;
        }
        "BYBIT" => {
            let p = pair.replace("_", "");
            bybit::subscribe_to_pair(p, level, print_msg).await;
        }
        "KRAKEN" => {
            let p = pair.replace("_", "/");
            kraken::subscribe_to_pair(p, level, print_msg).await;
        }
        "POLONIEX" => {
            poloniex::subscribe_to_pair(pair, level, print_msg).await;
        }
        _ => {
            println!("unknown exhange: {}", exchange)
        }
    }
}

fn print_msg(value: Value) {
    println!("the ticker: {:?}", value);
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut feeds = Vec::new();

    for f in cli.feed.into_iter() {
        println!("feed: {:?}", f);
        let parts: Vec<&str> = f.split("__").collect();
        let pair_level: Vec<&str> = parts[1].split(":").collect();

        let mut pair = pair_level[0].to_uppercase().to_owned();
        let level: u8 = pair_level[1].parse().unwrap();
        let exchange = parts[0].to_uppercase();

        feeds.push(subscribe_to_pair(exchange, pair, level, print_msg));
    }

    join_all(feeds).await;
}
