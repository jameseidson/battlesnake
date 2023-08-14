use battlesnake::{engine, server};
use tokio;

#[tokio::main]
async fn main() {
    let dumb_engine = engine::Dumb::new();
    server::serve(&"127.0.0.1:8000".parse().unwrap(), dumb_engine).await;
}
