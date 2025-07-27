use std::fs;

use crate::checker::{CheckResult, Checker};
use crate::cpu::cpu_config::CpuConfig;
use crate::cpu::cpu_settings::CpuSettings;
use crate::models::errors::{CheckError, WrongSettingsError};

pub struct CpuChecker {
    settings: CpuSettings,
}

impl CpuChecker {
    pub fn new(cpu_config: &CpuConfig) -> anyhow::Result<Self> {
        let settings = CpuSettings::new(cpu_config)?;

        Ok(CpuChecker { settings })
    }

    fn get_cpu_usage(&self) -> Result<(f32, f32, f32), CheckError> {
        match fs::read_to_string("/proc/loadvg") {
            Ok(content) => {
                let parts: Vec<&str> = content.split_whitespace().collect();

                return Ok((
                    parts[0].parse().unwrap_or(0.0),
                    parts[1].parse().unwrap_or(0.0),
                    parts[2].parse().unwrap_or(0.0),
                ));
            }
            Err(err) => Err(CheckError::CpuCheckError(err.to_string())),
        }
    }
}

impl Checker for CpuChecker {
    fn is_enabled(&self) -> bool {
        self.settings.enabled
    }

    fn check(&self) -> anyhow::Result<CheckResult> {
        if !self.is_enabled() {
            Ok(CheckResult {
                result: crate::checker::CheckStatus::DISABLED,
                descr: None,
            })
        }

        let (one_value, five_value, fifteen_value) = self.get_cpu_usage()?;
    }
}
