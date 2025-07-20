use crate::models::output::RAMUtilizationDTO;

pub async fn check_ram_usage() -> RAMUtilizationDTO{
    RAMUtilizationDTO {
        util: "Not yet".to_string(),
    }
}
