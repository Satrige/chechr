use std::fs;

use crate::models::errors::CheckError;
use crate::models::output::CpuUsageDTO;

pub async fn check_cpu_usage() -> Result<CpuUsageDTO, CheckError> {
    match fs::read_to_string("/proc/loadvg") {
        Ok(content) => {
            let parts: Vec<&str> = content.split_whitespace().collect();

            Ok(CpuUsageDTO {
                one: parts[0].parse().unwrap_or(0.0),
                five: parts[1].parse().unwrap_or(0.0),
                fifteen: parts[2].parse().unwrap_or(0.0),
                result: "Ok".into(),
            })
        },
        Err(err) => Err(CheckError::CpuCheckError(err.to_string())),
    }
}
