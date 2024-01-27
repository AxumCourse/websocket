use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
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

async fn handle_socket(socket: WebSocket) {
    // 拆分 WebSocket 流
    let (sender, mut receiver) = socket.split();

    // 接收消息
    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => {
                    println!("客户端断开连接");
                    break;
                }
                Message::Text(text) => {
                    println!("收到客户端文本消息：{}", text);
                    // 将接收到的消息传递给发送句柄
                }
                _ => println!("收到客户端消息：{:?}", msg),
            };
        }
    });

    // 发送消息
    tokio::spawn(async move {
        // 从接收句柄接收消息
        // 然后将消息原样发送给客户端
    });
}
