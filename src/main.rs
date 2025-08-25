use crate::adapters::{BinanceAdapter, ExchangeAdapter};

mod adapters;

fn main() {
    let binance = BinanceAdapter::new();
    let markets = binance.get_markets();
    
    println!("{:?} markets: {:?}", binance.name() ,markets);
}
