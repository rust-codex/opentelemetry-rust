use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::protobuf::Protobuf;
use futures::{sink::SinkExt, stream::StreamExt};
use opentelemetry_proto::tonic::logs::v1::LogsData;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::broadcast;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

struct AppState {
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).pretty().init();

    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { tx });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/v1/logs", post(otel_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    trace::DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(true),
                )
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn otel_handler(
    State(state): State<Arc<AppState>>,
    Protobuf(payload): Protobuf<LogsData>,
) -> impl IntoResponse {
    info!("{:?}", serde_json::to_string(&payload).unwrap());
    let _ = state.tx.send(serde_json::to_string(&payload).unwrap());

    (StatusCode::OK, "Ok")
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, state: Arc<AppState>) {
    let (mut sender, _receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    })
    .await
    .unwrap();

    println!("Websocket context {who} destroyed");
}
