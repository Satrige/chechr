use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum CheckError {
    #[error("Cpu check error: {0}")]
    CpuCheckError(String),
    #[error("Ram check error: {0}")]
    RamCheckError(String),
}
