# arbitrage-monitor

A learning project in Rust. 95% manual coding. Using claude as a teacher. App pulls bid/ask tickers from several crypto
exchanges and finds price spreads between them that could be arbitrage opportunities.

## What works

- **HTTP ticker adapters** for Binance, Bybit, OKX, Gate, Bitget and KuCoin. All of them implement the shared
  `ExchangeAdapter` trait (`src/adapters/common/traits.rs`).
- **Periodic monitor** (`src/monitor.rs`): it fetches tickers from every enabled exchange at the same time (`join_all`),
  then repeats every 30 seconds on a cron schedule.
- **Spread calculation** with Polars (`src/adapters/common/ticker_dataset.rs`): it joins the tickers from each pair of
  exchanges, filters by blacklist and minimum volume, and prints the entry and exit spreads above a threshold.
- Structured logging with `tracing`.

## Work in progress

- **WebSockets** (`src/adapters/common/websockets.rs`): there's a `WebSocketManager` that opens several connections and
  merges their messages into one channel. It isn't wired into the adapters yet. `ExchangeAdapter::orderbook_subscribe`
  is still `todo!()` and the Bybit payload builder is commented out.
- **Only perpetual swaps are fetched.** Spot fetching is commented out in `update_tickers`.
- **Only Bybit and OKX are enabled** in `ArbitrageMonitor::new`. The other adapters are commented out there.

## Run

```sh
cargo run --bin arbitrage-monitor   # main monitor (prints spread tables)
cargo run --bin websocket-test      # WebSocket sandbox (connects to Bybit, doesn't subscribe yet)
```
