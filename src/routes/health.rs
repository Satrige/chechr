use axum::{
    Json,
    routing::get,
    http::StatusCode,
    Router,
};
use tokio::join;
use crate::handlers::{
    cpu::check_cpu_usage,
    ram::check_ram_usage,
};
use crate::models::output::HealthDTO;

pub fn routes() -> Router {
    Router::new().route(
        "/", get(handler)
    )
}

async fn handler() -> (StatusCode, Json<HealthDTO>) {
    let (cpu_result, ram_result) = join!(
        check_cpu_usage(),
        check_ram_usage(),
    );

    (StatusCode::OK, Json(HealthDTO {
        cpu: cpu_result,
        ram: ram_result,
    }))
}
