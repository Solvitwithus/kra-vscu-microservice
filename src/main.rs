mod web;

use reqwest::Method;
use web::routing::route_sales;
mod types;
use axum::{Router, serve};
use dotenvy::dotenv;
use sea_orm::Database;
use std::env;
use anyhow::Result;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tokio::net::TcpListener;
mod models;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling;
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    dotenv().ok();
 init_tracing();

 tracing::info!("Starting VSCU middleware service");
    let database_url = env::var("DATABASE_URL")?;
    let db = Database::connect(&database_url).await?;

    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

   
    let app = Router::new()
        .nest("/save-sales-data", route_sales(db.clone()))
        .layer(cors)
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
        );
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Server listening on {}", addr);

    serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await?;

    Ok(())
}

async fn shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    println!("Signal received, shutting down gracefully");
}


fn init_tracing() {
    // logs/2026-01-16.log
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .with_writer(non_blocking)
        .with_ansi(false) // disable colors for files
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // IMPORTANT: prevent log file from closing
    std::mem::forget(_guard);
}