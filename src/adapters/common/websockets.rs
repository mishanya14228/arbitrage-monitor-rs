use super::types::MarketType;

struct WebSocketClient {
    url: String,
    market_type: MarketType,
}

impl WebSocketClient {
    pub fn new(url: String, market_type: MarketType) -> Self {
        Self { url, market_type }
    }

    pub fn connect(&mut self) {}
}

struct WebSocketManager {
    clients: Vec<WebSocketClient>,
}

impl WebSocketManager {
    pub fn new(clients: Vec<WebSocketClient>) -> Self {
        Self { clients }
    }

    pub fn connect_all(&mut self) {
        todo!()
    }
    pub fn listen_symbols(&mut self, symbol: Vec<&str>) {
        todo!()
    }
}

fn asd() {
    let mut manager = WebSocketManager::new(vec![
        WebSocketClient {
            url: "httpt:some-endpoint".to_string(),
            market_type: MarketType::Spot,
        },
        WebSocketClient {
            url: "httpt:some-endpoint".to_string(),
            market_type: MarketType::Spot,
        },
    ]);
    manager.connect_all();
    let symbols: Vec<&str> = ["BTC/USDT", "BTC/USDT:USDT"].into();
    manager.listen_symbols(symbols)
}
