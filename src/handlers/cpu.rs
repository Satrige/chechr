use crate::models::errors::CheckError;
use crate::models::output::CpuUsageDTO;

pub async fn check_cpu_usage() -> Result<CpuUsageDTO, CheckError> {
    Ok(
        CpuUsageDTO {
            usage: "Not yet".to_string(),
        }
    )
}
