use crate::{config::AppConfig, cpu::cpu_checker::CpuChecker, models::checker::Checker};
use std::sync::Arc;

pub fn get_checkers(config: &AppConfig) -> anyhow::Result<Vec<Arc<dyn Checker + Send + Sync>>> {
    let mut result: Vec<Arc<dyn Checker + Send + Sync>> = Vec::new();

    if let Some(cpu_config) = &config.cpu {
        let cpu_checker = CpuChecker::new(cpu_config)?;
        result.push(Arc::new(cpu_checker) as Arc<dyn Checker + Send + Sync>);
    }

    Ok(result)
}
