#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarketType {
    Spot,
    Swap,
}

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::broadcast::{self, Receiver as BroadcastReceiver, Sender as BroadcastSender};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::{Error as WsError, Message};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{error, info};

enum SubscriptionType {
    Orderbook(String),
}

pub struct WebSocketClient {
    url: String,
    market_type: MarketType,
    writer: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    listener_handle: Option<JoinHandle<()>>,
    events_tx: BroadcastSender<Message>,

    pub subscriptions: Vec<SubscriptionType>,
}

impl WebSocketClient {
    pub fn new(url: String, market_type: MarketType) -> Self {
        // Each client gets a broadcast channel so multiple consumers can tap into
        let (events_tx, _events_rx) = broadcast::channel(1024);

        Self {
            url,
            market_type,
            writer: None,
            listener_handle: None,
            events_tx,
            subscriptions: vec![],
        }
    }

    pub async fn connect(&mut self) -> Result<(), WsError> {
        let (ws_stream, _) = connect_async(self.url.clone()).await?;

        let (writer, mut reader) = ws_stream.split();
        self.writer = Some(writer);
        let events_tx = self.events_tx.clone();
        self.listener_handle = Some(tokio::spawn(async move {
            while let Some(msg) = reader.next().await {
                match msg {
                    Ok(msg) => {
                        if events_tx.send(msg).is_err() {
                            break; // no receivers left
                        }
                    }
                    Err(e) => {
                        error!("Listen error: {}", e);
                        break;
                    }
                }
            }
            info!("Listener stopped");
        }));
        Ok(())
    }

    pub fn subscribe_events(&self) -> BroadcastReceiver<Message> {
        self.events_tx.subscribe()
    }

    pub async fn close(&mut self) {
        if let Some(mut writer) = self.writer.take() {
            let _ = writer.send(Message::Close(None)).await;
        }
        if let Some(handle) = self.listener_handle.take() {
            handle.abort();
        }
    }

    pub async fn send<T>(&mut self, msg: T)
    where
        T: Into<Message>,
    {
        if let Some(writer) = &mut self.writer {
            if let Err(e) = writer.send(msg.into()).await {
                error!("Send error: {}", e);
            }
        } else {
            error!("Not connected");
        }
    }

}

pub struct WebSocketManager {
    clients: Vec<Arc<Mutex<WebSocketClient>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self { clients: vec![] }
    }
}

trait WebSocketManagerTrait {
    fn get_clients(&self) -> &Vec<Arc<Mutex<WebSocketClient>>>;
    fn push_client(&mut self, client: Arc<Mutex<WebSocketClient>>);

    async fn create_connection(
        &mut self,
        url: String,
        market_type: MarketType,
    ) -> Result<Arc<Mutex<WebSocketClient>>, WsError> {
        /*
         * CREATE AND SETUP A NEW WEBSOCKET CLIENT
         *
         * In JavaScript, you might do:
         * const connection = new WebSocket(url);
         * connection.addEventListener('open', () => console.log('Connected'));
         *
         * But in Rust with multiple tasks, we need to wrap it in Arc<Mutex<T>> for sharing:
         */

        // 1. Create a new WebSocket client (like: new WebSocket(url))
        let mut connection = WebSocketClient::new(url, market_type);

        // 2. Actually connect to the WebSocket server (like waiting for 'open' event)
        connection.connect().await?;

        // 3. Wrap in Arc<Mutex<T>> for safe sharing across async tasks
        // Arc = "Atomic Reference Counter" - like a smart pointer that can be cloned safely
        // Mutex = "Mutual Exclusion" - ensures only one task can access the client at a time
        //
        // In JS terms: this is like making the WebSocket connection thread-safe
        // so multiple async functions can use it without stepping on each other
        let client_ref = Arc::new(Mutex::new(connection));

        // 4. Store this client in our manager's list (like: this.clients.push(client))
        self.push_client(client_ref.clone());

        // 5. Return a clone of the Arc - this is cheap! Just incrementing a reference counter
        // The caller gets their own "handle" to the same underlying WebSocket client
        Ok(client_ref)
    }

    async fn orderbook_subscribe(&self, symbols: Vec<String>) -> UnboundedReceiver<Message> {
        /*
         * GOAL: Create a single stream that merges messages from multiple WebSocket clients
         * Think of this like Promise.all() but for streams - we want to listen to multiple
         * WebSocket connections simultaneously and get all their messages in one place.
         */

        // 1. Create a "unified channel" - this is like creating a single EventEmitter
        //    that will receive messages from all our WebSocket clients
        //    In JS terms: const unifiedEmitter = new EventEmitter();
        let (unified_tx, unified_rx) = mpsc::unbounded_channel();

        // 2. For each WebSocket client we have, create a separate background task
        //    This is similar to starting multiple fetch() requests in parallel
        for (client_idx, client) in self.get_clients().iter().enumerate() {
            // Clone references so each background task can own them
            // In JS: const clientCopy = client; (but Rust needs explicit cloning for thread safety)
            let client_clone = client.clone(); // Clone the Arc<Mutex<WebSocketClient>>
            let symbols_clone = symbols.clone(); // Clone the symbols array
            let unified_tx_clone = unified_tx.clone(); // Clone the sender (like copying a callback function)

            // 3. Start a background task for this client (like setTimeout or setImmediate in Node.js)
            //    Each task will:
            //    a) Send subscription message to its WebSocket
            //    b) Listen for incoming messages
            //    c) Forward those messages to our unified channel
            tokio::spawn(async move {
                println!("Starting background task for client {}", client_idx);

                // STEP A: Send the subscription message to this specific WebSocket client
                {
                    // Get exclusive access to this client (like acquiring a lock in multithreaded JS)
                    // .await here because tokio::Mutex is async-aware (unlike std::Mutex)
                    let mut client_guard = client_clone.lock().await;

                    // Build the subscription message for this exchange's format
                    // Transform ["BTCUSDT", "ETHUSDT"] -> ["orderbook.50.BTCUSDT", "orderbook.50.ETHUSDT"]
                    let orderbook_symbols: Vec<String> = symbols_clone
                        .iter()
                        .map(|s| format!("orderbook.50.{}", s))
                        .collect();

                    // Create JSON subscription message like:
                    // {"op":"subscribe", "req_id": 0, "args":["orderbook.50.BTCUSDT", "orderbook.50.ETHUSDT"]}
                    let subscription_msg = format!(
                        r#"{{"op":"subscribe", "req_id": {}, "args":[{}]}}"#,
                        client_idx,
                        orderbook_symbols
                            .iter()
                            .map(|s| format!(r#""{}""#, s)) // Wrap each symbol in quotes
                            .collect::<Vec<_>>()
                            .join(",") // Join with commas
                    );

                    // Send the subscription message over WebSocket
                    // This is like: websocket.send(JSON.stringify(subscriptionMsg))
                    client_guard.send(subscription_msg).await;

                    println!("Client {} subscribed to symbols", client_idx);
                } // Lock is automatically released here (like finally block)

                // STEP B: Grab a fresh broadcast receiver for this client
                let mut rx = {
                    let client_guard = client_clone.lock().await;
                    client_guard.subscribe_events()
                };

                println!("Client {} starting to forward messages", client_idx);

                // STEP C: Listen for messages and forward them to the unified channel
                // This is like: messageStream.on('message', (msg) => unifiedEmitter.emit('message', msg))
                while let Ok(message) = rx.recv().await {
                    if unified_tx_clone.send(message).is_err() {
                        println!("Client {}: Unified receiver dropped, stopping", client_idx);
                        break;
                    }
                }

                info!("Client {} stream ended", client_idx);
            });
        }

        // 4. Drop our copy of the sender
        // This is crucial! Once all the background tasks finish, the unified_rx will close
        // It's like saying: "When all WebSocket connections close, close the unified stream too"
        // In JS terms: when all event listeners are removed, stop the EventEmitter
        drop(unified_tx);

        // 5. Return the receiver end - this is what the caller will use to get messages
        // The caller can now do: while let Some(msg) = stream.recv().await { ... }
        // In JS terms: return unifiedEmitter; (and the caller does unifiedEmitter.on('message', ...))
        unified_rx
    }

    fn orderbook_unsubscribe(&self) {}
}

impl WebSocketManagerTrait for WebSocketManager {
    fn get_clients(&self) -> &Vec<Arc<Mutex<WebSocketClient>>> {
        &self.clients
    }

    fn push_client(&mut self, client: Arc<Mutex<WebSocketClient>>) {
        self.clients.push(client);
    }

    // fn get_clients(&mut self) -> HashMap<String, Vec<WebSocketClient>> {
    // self.clients.clone()
    // }
}

#[tokio::main]
async fn main() {
    const URL: &str = "wss://stream.bybit.com/v5/public/linear";
    let mut manager = WebSocketManager::new();

    // Create multiple connections to handle rate limits
    // In practice, you might use different URLs or endpoints
    for i in 0..2 {
        println!("Creating client {}", i);
        if let Err(err) = manager
            .create_connection(URL.to_string(), MarketType::Swap)
            .await
        {
            error!("Failed to create client {}: {}", i, err);
        }
    }

    // Subscribe to symbols across all clients
    let symbols = vec![
        "BTCUSDT".to_string(),
        // "ETHUSDT".to_string(),
        "SOLUSDT".to_string(),
    ];
    let mut unified_stream = manager.orderbook_subscribe(symbols).await;

    // Listen to the unified stream
    println!(
        "Listening to unified stream from {} clients",
        manager.get_clients().len()
    );
    while let Some(message) = unified_stream.recv().await {
        if let tokio_tungstenite::tungstenite::Message::Text(text) = message {
            println!("Received: {}", text);
        }
    }
}

// async fn interrupt(client: Arc<Mutex<WebSocketClient>>) {
//     tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//     println!("Stopping...");
//     let msg = r#"{"op":"unsubscribe", "req_id": 123,"args":["orderbook.50.BTCUSDT", "orderbook.50.ETHUSDT", "orderbook.50.SOLUSDT"]}"#;
//     {
//         let mut client_guard = client.lock().await;
//         client_guard.send(msg).await;
//     }
//     println!("unsubscribed");
//     tokio::time::sleep(std::time::Duration::from_secs(2)).await;
//     {
//         let mut client_guard = client.lock().await;
//         client_guard.close().await;
//     }
//     println!("closed");
// }
