use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::event::Ticker;

pub async fn subscribe_to_pair<F>(pair: String, level: u8, cb: F)
where
    F: Fn(Value),
{
    let mut ws_url = format!(
        "wss://stream.binance.com:9443/ws/{}@bookTicker",
        pair.to_lowercase()
    );

    match level {
        1 => {
            ws_url = format!(
                "wss://stream.binance.com:9443/ws/{}@bookTicker",
                pair.to_lowercase()
            );
        }
        2 => {
            ws_url = format!(
                "wss://stream.binance.com:9443/ws/{}@depth",
                pair.to_lowercase()
            );
        }
        3 => {
            println!("unrecognized level: {}", level);
        }
        _ => {
            println!("unrecognized level: {}", level);
        }
    }

    println!("{}", ws_url);

    let request = ws_url.into_client_request().unwrap();
    let (ws_stream, _) = connect_async(request).await.unwrap();

    let (mut write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    match serde_json::from_str::<Value>(&msg.into_text().unwrap()) {
                        Ok(json) => {
                            // println!("Parsed JSON: {:?}", json);
                            match level {
                                1 => {
                                    let ask_qty: f32 = json["A"].as_str().unwrap().parse().unwrap();
                                    let bid_qty: f32 = json["B"].as_str().unwrap().parse().unwrap();
                                    let ask_price: f32 =
                                        json["a"].as_str().unwrap().parse().unwrap();
                                    let bid_price: f32 =
                                        json["b"].as_str().unwrap().parse().unwrap();
                                    let seq: u64 = json["u"].as_i64().unwrap() as u64;
                                    let t = Ticker::new(
                                        "binance".to_string(),
                                        pair.to_string(),
                                        ask_price,
                                        ask_qty,
                                        bid_price,
                                        bid_qty,
                                        seq,
                                    );
                                    cb(json);
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                        }
                    }
                } else if msg.is_close() {
                    println!("Received close message");
                    break;
                } else if msg.is_ping() {
                    println!("Received ping message");
                    if let Err(e) = write.send(Message::Pong(msg.into_data())).await {
                        eprintln!("Failed to send Pong: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
}
