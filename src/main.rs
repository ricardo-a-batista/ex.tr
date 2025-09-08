use axum::{Router, routing::get};
use expenses_tracker::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(async || "Hello World!"));

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
