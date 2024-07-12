use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use yew_agent::reactor::{reactor, ReactorScope};

#[reactor(WebsocketReactor)]
pub async fn websocket_reactor(mut scope: ReactorScope<u64, Vec<String>>) {
    let ws = WebSocket::open("ws://127.0.0.1:3000/ws").unwrap();
    let mut cache: Vec<String> = Vec::new();

    let (mut _write, mut read) = ws.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(data)) => {
                log::debug!("from websocket: {}", &data);

                cache.push(data);
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
