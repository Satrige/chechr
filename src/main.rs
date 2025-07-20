use axum::Router;
use tracing_subscriber;

mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Started logging");

     // build our application with a route
    let app = Router::new()
        .nest("/health", routes::health::routes());
    

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
