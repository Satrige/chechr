use serde::Serialize;

#[derive(Serialize)]
pub struct CpuUsageDTO {
    pub one: Option<f32>,
    pub five: Option<f32>,
    pub fifteen: Option<f32>,
    pub result: String,
}

#[derive(Serialize)]
pub struct RAMUtilizationDTO {
    pub util: String,
}

#[derive(Serialize)]
pub struct HealthDTO {
    pub cpu: CpuUsageDTO,
    pub ram: RAMUtilizationDTO,
}
