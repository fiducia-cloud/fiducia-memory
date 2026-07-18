//! Runtime wiring for the legacy durable-memory HTTP service.

use std::{net::SocketAddr, time::Duration};

use axum::http::StatusCode;
use sea_orm::{ConnectOptions, Database};
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer};

use crate::{router, store::MemoryStore};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be configured")?;
    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(20)
        .connect_timeout(Duration::from_secs(5));
    let database = Database::connect(options).await?;
    let store = MemoryStore::new(database);
    store.migrate().await?;

    let app = router(store)
        .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(TraceLayer::new_for_http());
    let address: SocketAddr = std::env::var("FIDUCIA_MEMORY_BIND")
        .unwrap_or_else(|_| "127.0.0.1:8090".into())
        .parse()?;
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(%address, database.orm = "sea-orm", "fiducia-memory listening");
    axum::serve(listener, app).await?;
    Ok(())
}
