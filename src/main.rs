use axum::Router;
use clap::Parser;

mod cpu;

mod config;
mod helpers;
mod logger;
mod models;
mod routes;

use config::{AppConfig, Args};
use logger::set_up_logger;

use crate::{helpers::get_checkers::get_checkers, routes::health::HealthRouters};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = AppConfig::from_file(&args.config)?;

    set_up_logger(&config);

    tracing::info!("Started chechr on port: {}", config.port);

    let checkers = get_checkers(&config)?;

    let app = Router::new().nest("/health", HealthRouters::new(checkers).get_rountes());

    let port = config.port;
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
