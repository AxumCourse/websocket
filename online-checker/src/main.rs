use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use online_checker::{config::Config, http_handler, ws_handler, AppState};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = Config::from_env().unwrap();

    let listener = TcpListener::bind(&cfg.web_addr).await.unwrap();

    tracing::info!("服务监听于 {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/login", post(http_handler::login))
        .route("/logout", get(http_handler::logout))
        .route("/check", get(ws_handler::online))
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .with_state(Arc::new(AppState { cfg }));

    axum::serve(listener, app).await.unwrap()
}
