use super::types::{ExchangeMarketsInfo, MarketType, TickerBidAsk};
use anyhow::Error;
use async_trait::async_trait;
use tracing::instrument;

#[async_trait]
pub trait ExchangeAdapter: Send {
    fn markets(&self) -> &ExchangeMarketsInfo;
    fn markets_mut(&mut self) -> &mut ExchangeMarketsInfo;

    fn name(&self) -> &str;

    fn wrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        match market_type {
            MarketType::Swap => {
                format!("{}USDT", symbol.strip_suffix("/USDT:PERP").unwrap_or(""))
            }
            MarketType::Spot => {
                format!("{}USDT", symbol.strip_suffix("/USDT").unwrap_or(""))
            }
        }
    }

    fn unwrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        let base = symbol.strip_suffix("USDT").unwrap_or("");
        match market_type {
            MarketType::Swap => {
                format!("{}/USDT:PERP", base)
            }
            MarketType::Spot => {
                format!("{}/USDT", base)
            }
        }
    }

    fn get_spot_markets(&self) -> Vec<TickerBidAsk> {
        self.markets().spot.clone()
    }

    fn set_spot_markets(&mut self, markets: Vec<TickerBidAsk>) {
        self.markets_mut().spot = markets;
    }

    fn get_swap_markets(&self) -> Vec<TickerBidAsk> {
        self.markets().swap.clone()
    }

    fn set_swap_markets(&mut self, markets: Vec<TickerBidAsk>) {
        self.markets_mut().swap = markets;
    }

    async fn fetch_spot_tickers(
        &self,
        tickers: Option<Vec<&str>>,
    ) -> Result<Vec<TickerBidAsk>, anyhow::Error>;
    async fn fetch_swap_tickers(
        &self,
        tickers: Option<&str>,
    ) -> Result<Vec<TickerBidAsk>, anyhow::Error>;

    #[instrument(level = "info", skip(self), fields(adapter = self.name()))]
    async fn update_tickers(&mut self) -> Result<Vec<TickerBidAsk>, Error> {
        let (spot_fetch_result, swap_fetch_result) =
            tokio::join!(self.fetch_spot_tickers(None), self.fetch_swap_tickers(None));

        let spot_tickers = spot_fetch_result?;
        let swap_tickers = swap_fetch_result?;
        self.set_spot_markets(spot_tickers.clone());
        self.set_swap_markets(swap_tickers.clone());
        // Return combined markets
        let mut all_tickers: Vec<TickerBidAsk> = spot_tickers;
        all_tickers.extend(swap_tickers);
        Ok(all_tickers)
    }
}
