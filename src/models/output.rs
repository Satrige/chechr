use serde::Serialize;

#[derive(Serialize)]
pub struct CpuUsageDTO {
    pub usage: String,
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
