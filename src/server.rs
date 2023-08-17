use crate::common::{GameState, SnakeInfo};
use crate::snakes::BattleSnake;
use axum::{extract::State, response, routing, Json, Router, Server};
use serde_json::{json, Value};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

pub async fn serve<E: BattleSnake + Send + 'static>(addr: &SocketAddr, engine: E) {
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

async fn handle_root<E: BattleSnake + Send + 'static>(
    State(engine): State<Arc<Mutex<E>>>,
) -> Json<SnakeInfo> {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    Json(engine.info())
}

async fn handle_start<E: BattleSnake + Send + 'static>(
    State(engine): State<Arc<Mutex<E>>>,
    Json(game_state): Json<GameState>,
) {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    engine.start(&game_state);
}

async fn handle_move<E: BattleSnake + Send + 'static>(
    State(engine): State<Arc<Mutex<E>>>,
    Json(game_state): Json<GameState>,
) -> response::Json<Value> {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    let action = engine.next(&game_state);

    Json(json!({ "move": action }))
}

async fn handle_end<E: BattleSnake + Send + 'static>(
    State(engine): State<Arc<Mutex<E>>>,
    Json(game_state): Json<GameState>,
) {
    let mutex = engine.clone();
    let engine = mutex.lock().await;

    engine.end(&game_state);
}
