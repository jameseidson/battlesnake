use crate::{engine::Engine, GameState};
use axum::{extract::State, response, routing, Json, Router, Server};
use serde_json::{json, Value};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

pub async fn serve<E: Engine>(addr: &SocketAddr, engine: E) {
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

async fn handle_start<E: Engine>(
    State(engine): State<Arc<Mutex<E>>>,
    Json(game_state): Json<GameState>,
) {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    engine.start(game_state);
}

async fn handle_move<E: Engine>(
    State(engine): State<Arc<Mutex<E>>>,
    Json(game_state): Json<GameState>,
) -> response::Json<Value> {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    let next_move = engine.next(game_state);

    Json(json!({ "move": next_move }))
}

async fn handle_end<E: Engine>(
    State(engine): State<Arc<Mutex<E>>>,
    Json(game_state): Json<GameState>,
) {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    engine.end(game_state);
}
