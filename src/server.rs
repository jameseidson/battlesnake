use crate::{engine::SnakeEngine, GameState};
use axum::{extract::State, response, routing, Json, Router, Server};
use serde_json::{json, Value};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

pub async fn serve<E: SnakeEngine + 'static + Send>(addr: &SocketAddr, engine: E) {
    let app = Router::new()
        .route("/", routing::get(handle_root))
        .route("/start", routing::post(handle_start))
        .route("/move", routing::post(handle_move))
        .route("/end", routing::post(handle_end))
        .with_state(Arc::new(Mutex::new(engine)));

    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_root() -> Json<Value> {
    response::Json(
        json!({ "apiversion": "1", "author" : "jameseidson", "version" : env!("CARGO_PKG_VERSION") }),
    )
}

async fn handle_start<E: SnakeEngine + 'static + Send>(
    State(engine): State<Arc<Mutex<E>>>,
    Json(game_state): Json<GameState>,
) {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    engine.start(&game_state);
}

async fn handle_move<E: SnakeEngine + 'static + Send>(
    State(engine): State<Arc<Mutex<E>>>,
    Json(game_state): Json<GameState>,
) -> response::Json<Value> {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    let action = engine.next(&game_state);

    Json(json!({ "move": action }))
}

async fn handle_end<E: SnakeEngine + 'static + Send>(
    State(engine): State<Arc<Mutex<E>>>,
    Json(game_state): Json<GameState>,
) {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    engine.end(&game_state);
}
