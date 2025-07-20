use axum::{
    Json,
    routing::get,
    http::StatusCode,
    Router,
};
use tokio;
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
    let cpu_handle = tokio::spawn(check_cpu_usage());
    let ram_handle = tokio::spawn(check_ram_usage());

    let cpu_result = cpu_handle.await.unwrap();
    let ram_result = ram_handle.await.unwrap();

    match (cpu_result, ram_result) {
        (Ok(cpu_res), Ok(ram_res)) => (StatusCode::OK, Json(HealthDTO {
            cpu: cpu_res,
            ram: ram_res,
        })),
        _ => panic!("Sth went wrong"),
    }
}
