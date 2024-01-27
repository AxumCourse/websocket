use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "simple_echo=debug");
    }
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("0.0.0.0:56789").await.unwrap();
    let app = Router::new().route("/ws", get(websocket_handler));
    info!("监听于 {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Close(_) => {
                info!("客户端断开连接");
                break;
            }
            Message::Text(text) => {
                info!("收到客户端文本消息：{}", text);
                // 向客户端原样发送收到的消息
                socket.send(Message::Text(text)).await.unwrap();
            }
            _ => info!("收到客户端消息：{:?}", msg),
        };
    }
}
