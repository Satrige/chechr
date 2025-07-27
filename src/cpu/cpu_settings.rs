use crate::{cpu::cpu_config::CpuConfig, models::errors::WrongSettingsError};

struct CpuThresholdSettings {
    one_threshold: f32,
    five_threshold: f32,
    fifteen_threshold: f32,
}

pub struct CpuSettings {
    pub enabled: bool,
    pub warning: Option<CpuThresholdSettings>,
    pub critical: Option<CpuThresholdSettings>,
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
