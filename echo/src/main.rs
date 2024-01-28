use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
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

async fn handle_socket(socket: WebSocket) {
    // 拆分 WebSocket 流
    let (sender, receiver) = socket.split();

    // 创建管道
    let (tx, rx) = tokio::sync::mpsc::channel::<Message>(100);

    // 接收消息异步任务
    tokio::spawn(read(receiver, tx));

    // 发送消息异步任务
    tokio::spawn(write(sender, rx));
}

/// 接收消息
async fn read(mut receiver: SplitStream<WebSocket>, tx: tokio::sync::mpsc::Sender<Message>) {
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Close(_) => {
                println!("客户端断开连接");
                break;
            }
            Message::Text(text) => {
                println!("收到客户端文本消息：{}", text);
                // 通过管道，将接收到的消息传递给发送句柄
                tx.send(Message::Text(text)).await.unwrap();
            }
            _ => println!("收到客户端消息：{:?}", msg),
        };
    }
}

/// 发送消息
async fn write(
    mut sender: SplitSink<WebSocket, Message>,
    mut rx: tokio::sync::mpsc::Receiver<Message>,
) {
    // 通过管道，从接收句柄接收消息
    while let Some(msg) = rx.recv().await {
        // 然后将消息原样发送给客户端
        sender.send(msg).await.unwrap();
    }
}
