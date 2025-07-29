use crate::{config::AppConfig, models::log_level::LogLevel};
use tracing_subscriber;

pub fn set_up_logger(config: &AppConfig) {
    tracing_subscriber::fmt()
        .with_max_level(config.log_level.as_ref().unwrap_or(&LogLevel::ERROR))
        .init();
}
