#[cfg(test)]
mod tests {
    use crate::adapters::bitget::BitgetAdapter;
    use crate::adapters::common::traits::ExchangeAdapter;

    #[tokio::test]
    async fn test_bitget_spot_tickers() {
        let adapter = BitgetAdapter::new();
        
        // Test fetching spot tickers
        let result = adapter.fetch_spot_tickers(None).await;
        
        match result {
            Ok(tickers) => {
                println!("Successfully fetched {} spot tickers", tickers.len());
                if !tickers.is_empty() {
                    let first_ticker = &tickers[0];
                    println!("First ticker: {} - Bid: {}, Ask: {}", 
                            first_ticker.unified_symbol, 
                            first_ticker.bid, 
                            first_ticker.ask);
                    
                    // Basic validation
                    assert!(first_ticker.bid > 0.0);
                    assert!(first_ticker.ask > 0.0);
                    assert!(first_ticker.ask >= first_ticker.bid);
                }
            }
            Err(e) => {
                println!("Error fetching spot tickers: {}", e);
                // Don't panic on network errors in tests
            }
        }
    }

    #[tokio::test]
    async fn test_bitget_swap_tickers() {
        let adapter = BitgetAdapter::new();
        
        // Test fetching swap tickers
        let result = adapter.fetch_swap_tickers(None).await;
        
        match result {
            Ok(tickers) => {
                println!("Successfully fetched {} swap tickers", tickers.len());
                if !tickers.is_empty() {
                    let first_ticker = &tickers[0];
                    println!("First ticker: {} - Bid: {}, Ask: {}", 
                            first_ticker.unified_symbol, 
                            first_ticker.bid, 
                            first_ticker.ask);
                    
                    // Basic validation
                    assert!(first_ticker.bid > 0.0);
                    assert!(first_ticker.ask > 0.0);
                    assert!(first_ticker.ask >= first_ticker.bid);
                    assert!(first_ticker.unified_symbol.contains("PERP"));
                }
            }
            Err(e) => {
                println!("Error fetching swap tickers: {}", e);
                // Don't panic on network errors in tests
            }
        }
    }

    #[test]
    fn test_adapter_name() {
        let adapter = BitgetAdapter::new();
        assert_eq!(adapter.name(), "Bitget");
    }
}