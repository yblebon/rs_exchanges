use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use serde_json::Value;
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parameters
    let args: Vec<String> = env::args().collect();
    let exchange = &args[1];
    let pair = &args[2];
    let mut ws_url = format!("wss://stream.binance.com:9443/ws/{}@bookTicker", pair);

    match args[1].as_str() {
        "binance" => {
            println!("Binance!");
            ws_url = format!("wss://stream.binance.com:9443/ws/{}@bookTicker", pair);
        }
        "bybit" => {
            println!("ByBit!");
            ws_url = format!("wss://stream.bybit.com/v5/public/spot");
        }
        _ => {
            println!("Unknown command: {}", args[1])
        }
    }

    // Define the WebSocket URL to connect to
    let url = Url::parse(&ws_url)?;

    // Connect to the WebSocket server
    let (ws_stream, _) = connect_async(url).await?;
    println!("WebSocket handshake has been successfully completed");

    // Split the WebSocket stream into a sink and a stream
    let (mut write, mut read) = ws_stream.split();

    // send message
    let msg = json!({"req_id": "test", "op": "subscribe", "args": ["orderbook.1.BTCUSDT"]});
    if let Err(e) = write.send(Message::Text(msg.to_string())).await {
        eprintln!("Failed to send message: {}", e);
    }

    // Handle incoming messages
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

    Ok(())
}
