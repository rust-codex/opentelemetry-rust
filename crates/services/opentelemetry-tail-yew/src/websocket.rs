use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use serde::{Deserialize, Serialize};
use yew_agent::reactor::{reactor, ReactorScope};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Log {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

#[reactor(WebsocketReactor)]
pub async fn websocket_reactor(mut scope: ReactorScope<u64, Vec<Log>>) {
    let ws = WebSocket::open("ws://127.0.0.1:3000/ws").unwrap();
    let mut cache: Vec<Log> = Vec::new();

    let (mut _write, mut read) = ws.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(data)) => {
                log::debug!("from websocket: {}", &data);

                let mut logs: Vec<Log> = serde_json::from_str(&data).expect("invalid json");
                cache.append(&mut logs);

                if scope.send(cache.clone()).await.is_err() {
                    // sender closed, the bridge is disconnected
                    break;
                }
            }
            Ok(Message::Bytes(b)) => {
                let decoded = std::str::from_utf8(&b);
                if let Ok(val) = decoded {
                    log::debug!("from websocket: {}", val);
                }
            }
            Err(e) => {
                log::error!("ws: {:?}", e)
            }
        }
    }
    log::debug!("WebSocket Closed");
}
