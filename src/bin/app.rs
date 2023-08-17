use battlesnake::{server, snakes};
use tokio;

#[tokio::main]
async fn main() {
    let dumb_snake = snakes::Dumb::new();
    server::serve(&"127.0.0.1:8000".parse().unwrap(), dumb_snake).await;
}
