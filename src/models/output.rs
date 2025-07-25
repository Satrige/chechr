use serde::Serialize;

#[derive(Serialize)]
pub struct CpuUsageDTO {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
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
