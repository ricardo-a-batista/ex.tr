use std::sync::Arc;

use axum::{Router, routing::get};
use expenses_tracker::Result;
use expenses_tracker::controller::home;
use expenses_tracker::infra::{MIGRATOR, StateBuilder};
use expenses_tracker::instrument;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    instrument::init();

    let pool = SqlitePool::connect("sqlite:et.db?mode=rwc").await?;
    MIGRATOR.run(&pool).await?;

    let state = Arc::new(StateBuilder::default().with_pool(pool).build().await?);

    let app = Router::new()
        .route("/", get(home::index))
        .nest_service("/statics", ServeDir::new("statics"))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    info!("Expense Tracker server starting @ {}", address);
    axum::serve(listener, app).await?;

    Ok(())
}
