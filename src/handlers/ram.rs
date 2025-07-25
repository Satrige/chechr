use crate::models::errors::CheckError;
use crate::models::output::RAMUtilizationDTO;

pub async fn check_ram_usage() -> Result<RAMUtilizationDTO, CheckError> {
    Ok(RAMUtilizationDTO {
        util: "Not yet".to_string(),
    })
}
