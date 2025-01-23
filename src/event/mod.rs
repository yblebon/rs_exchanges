use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Ticker {
    exchange: String,
    pair: String,
    ask_price: f32,
    ask_qty: f32,
    bid_price: f32,
    bid_qty: f32,
    seq: u64,
    created_at: i64,
}

impl Ticker {
    pub fn new(
        exchange: String,
        pair: String,
        ask_price: f32,
        ask_qty: f32,
        bid_price: f32,
        bid_qty: f32,
        seq: u64,
    ) -> Self {
        Ticker {
            exchange,
            pair,
            ask_price,
            ask_qty,
            bid_price,
            bid_qty,
            seq,
            created_at: Utc::now().timestamp() as i64,
        }
    }
}
