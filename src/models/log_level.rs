use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}