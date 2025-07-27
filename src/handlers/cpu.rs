use std::fs;

use crate::config::cpu_config::CpuConfig;
use crate::models::errors::{CheckError, WrongSettingsError};
use crate::models::output::CpuUsageDTO;

struct CpuThresholdSettings {
    one_threshold: f32,
    five_threshold: f32,
    fifteen_threshold: f32,
}

struct CpuSettings {
    enabled: bool,
    warning: Option<CpuThresholdSettings>,
    critical: Option<CpuThresholdSettings>,
}

impl Default for CpuSettings {
    fn default() -> Self {
        CpuSettings {
            enabled: true,
            warning: Some(CpuThresholdSettings {
                one_threshold: 0.5,
                five_threshold: 0.5,
                fifteen_threshold: 0.5,
            }),
            critical: Some(CpuThresholdSettings {
                one_threshold: 1.0,
                five_threshold: 1.0,
                fifteen_threshold: 1.0,
            }),
        }
    }
}

impl CpuSettings {
    pub fn new(cpu_config: &Option<CpuConfig>) -> Result<Self, WrongSettingsError> {
        match cpu_config {
            Some(cpu_conf_content) => {
                // The case the check is explicitly disabled
                if let Some(cpu_check_enabled) = cpu_conf_content.enabled
                    && !cpu_check_enabled
                {
                    return Ok(CpuSettings {
                        enabled: false,
                        warning: None,
                        critical: None,
                    });
                }

                // The case the check is explicitly enabled but the thresholds are absent
                if cpu_conf_content.warning.is_none() || cpu_conf_content.critical.is_none() {
                    return Err(WrongSettingsError::WrongCpuSettingsError(
                        "The thresholds are not specified".into(),
                    ));
                }

                let warning_thresholds = cpu_conf_content.warning.as_ref().unwrap();
                let critical_thresholds = cpu_conf_content.critical.as_ref().unwrap();

                Ok(CpuSettings {
                    enabled: true,
                    warning: Some(CpuThresholdSettings {
                        one_threshold: warning_thresholds.one_threshold,
                        five_threshold: warning_thresholds.five_threshold,
                        fifteen_threshold: warning_thresholds.fifteen_threshold,
                    }),
                    critical: Some(CpuThresholdSettings {
                        one_threshold: critical_thresholds.one_threshold,
                        five_threshold: critical_thresholds.five_threshold,
                        fifteen_threshold: critical_thresholds.fifteen_threshold,
                    }),
                })
            }
            None => Ok(Self::default()),
        }
    }
}

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
        }
        Err(err) => Err(CheckError::CpuCheckError(err.to_string())),
    }
}
