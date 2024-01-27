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

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:56789").await.unwrap();
    let app = Router::new().route("/ws", get(websocket_handler));
    println!("监听于 {}", listener.local_addr().unwrap());
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
                println!("客户端断开连接");
                break;
            }
            Message::Text(text) => {
                println!("收到客户端文本消息：{}", text);
                // 向客户端原样发送收到的消息
                socket.send(Message::Text(text)).await.unwrap();
            }
            _ => println!("收到客户端消息：{:?}", msg),
        };
    }
}
