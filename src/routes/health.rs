use crate::models::checker::Checker;
use crate::models::output::HealthDTO;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json;
use tokio;

pub fn routes(checkers: Vec<Box<dyn Checker>>) -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Result<impl IntoResponse, impl IntoResponse> {
    let cpu_handle = tokio::spawn(check_cpu_usage());
    let ram_handle = tokio::spawn(check_ram_usage());

    let cpu_result = cpu_handle.await.unwrap();
    let ram_result = ram_handle.await.unwrap();

    match (cpu_result, ram_result) {
        (Ok(cpu_res), Ok(ram_res)) => Ok((
            StatusCode::OK,
            Json(HealthDTO {
                cpu: cpu_res,
                ram: ram_res,
            }),
        )
            .into_response()),
        _ => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Internal Error" })),
        )
            .into_response()),
    }
}
