use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use serde_json::Value;
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use url::Url;

pub async fn subscribe_to_pair(pair: &str) {
    let mut ws_url = format!("wss://stream.binance.com:9443/ws/{}@bookTicker", pair);
    let mut request = ws_url.into_client_request().unwrap();
    let (ws_stream, _) = connect_async(request).await.unwrap();

    let msg = json!({"req_id": "test", "op": "subscribe", "args": ["orderbook.1.BTCUSDT"]});

    let (mut write, mut read) = ws_stream.split();

    if let Err(e) = write.send(Message::Text(msg.to_string())).await {
        eprintln!("Failed to send message: {}", e);
    }

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    match serde_json::from_str::<Value>(&msg.into_text().unwrap()) {
                        Ok(json) => {
                            println!("Parsed JSON: {:?}", json);
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
