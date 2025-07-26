use axum::Router;
use clap::Parser;

mod config;
mod handlers;
mod logger;
mod models;
mod routes;

use config::{AppConfig, Args};
use logger::set_up_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = AppConfig::from_file(&args.config)?;

    set_up_logger(&config);

    tracing::info!("Started chechr on port: {}", config.port);

    // build our application with a route
    let app = Router::new().nest("/health", routes::health::routes());

    let port = config.port;
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
