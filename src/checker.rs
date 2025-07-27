use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    OK,
    WARNING,
    CRITICAL,
    DISABLED,
}

#[derive(Serialize)]
pub struct CheckResult {
    result: CheckStatus,
    descr: Option<String>,
}

pub trait Checker {
    fn is_enabled(&self) -> bool;

    fn check(&self) -> anyhow::Result<CheckResult>;
}
