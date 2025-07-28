use crate::{config::AppConfig, cpu::cpu_checker::CpuChecker, models::checker::Checker};

pub fn get_checkers(config: &AppConfig) -> anyhow::Result<Vec<Box<impl Checker>>> {
    let mut result = Vec::new();

    if let Some(cpu_config) = &config.cpu {
        let cpu_checker = CpuChecker::new(&cpu_config)?;
        result.push(Box::new(cpu_checker));
    }

    Ok(result)
}
