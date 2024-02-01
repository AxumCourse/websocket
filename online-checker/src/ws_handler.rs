use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::{jwt, AppState};

pub async fn online(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(move |stream| online_stream_handler(stream, state))
}

async fn online_stream_handler(stream: WebSocket, state: Arc<AppState>) {
    let (mut sdr, mut rer) = stream.split();
    let (tx, mut rx) = mpsc::channel::<String>(100);

    tokio::spawn(async move {
        while let Some(Ok(msg)) = rer.next().await {
            match msg {
                Message::Close(_) => {
                    info!("客户端关闭连接");
                    break;
                }
                Message::Text(text) => {
                    info!("接收到文本消息：{}", text);
                    tx.send(text).await.unwrap();
                }
                _ => {
                    info!("接收到消息：{:?}", msg);
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(token) = rx.recv().await {
            // 解码
            let claims = match jwt::token::decode::<jwt::UserClaimsData>(
                &token,
                &state.cfg.jwt_secret_key,
            ) {
                Ok(c) => c,
                Err(e) => {
                    error!("decode token failed: {:?}", e);
                    break;
                }
            };
            let claims_json = serde_json::json!(claims).to_string();
            sdr.send(Message::Text(claims_json)).await.unwrap();
        }
    });
}
