use crate::models::output::CpuUsageDTO;

pub async fn check_cpu_usage() -> CpuUsageDTO {
    CpuUsageDTO {
        usage: "Not yet".to_string(),
    }
}
