pub struct Ticker {
 exchange: String,
 pair: String,
 ask_price: f64,
 ask_qty: f64,
 bid_price: f64,
 bid_qty: f64
}

impl Ticker {
  pub fn new(exchange: String, pair: String, ask_price: f64, ask_qty: f64, bid_price: f64, bid_qty: f64) -> Self {
     Ticker {exchange, pair, ask_price, ask_qty, bid_price, bid_qty}
  }
}
