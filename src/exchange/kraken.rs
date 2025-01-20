use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use serde_json::Value;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub async fn subscribe_to_pair(pair: &str, level: &u8) {
    let ws_url = format!("wss://ws.kraken.com/v2");
    let request = ws_url.into_client_request().unwrap();
    let (ws_stream, _) = connect_async(request).await.unwrap();

    // level
    let mut msg = json!({"method": "subscribe", "params": {"channel": "ticker", "symbol": [format!("{}", pair)]}});

    match level {
        1 => {
            println!("level1: ticker best bid and best qty");
        }
        2 => {
            println!("level2: ordered book aggregated by qty");
            msg = json!({"method": "subscribe", "params": {"channel": "book", "symbol": [format!("{}", pair)]}});
        }
        3 => {
            println!("level3: book individual orders");
            msg = json!({"method": "subscribe", "params": {"channel": "level3", "symbol": [format!("{}", pair)]}});
        }
        _ => {
            println!("unrecognized level: {}", level);
        }
    }

    let (mut write, mut read) = ws_stream.split();

    if let Err(e) = write.send(Message::Text(msg.to_string())).await {
        eprintln!("Failed to send subscription message: {}", e);
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
