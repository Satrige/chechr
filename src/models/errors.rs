use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum CheckError {
    #[error("Cpu check error: {0}")]
    CpuCheckError(String),
    // #[error("Ram check error: {0}")]
    // RamCheckError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Read config error: {0}")]
    ReadConfigError(String),

    #[error("Parse config error: {0}")]
    ParseConfigError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum WrongSettingsError {
    #[error("Wrong CPU settings: {0}")]
    WrongCpuSettingsError(String),
}
