use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CpuConfig {
    pub enabled: Option<bool>,
    pub five_threshold: Option<f32>,
    pub ten_threshold: Option<f32>,
    pub fifteen_threshold: Option<f32>,
}
