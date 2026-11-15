use crate::error::AppError;
use database_environment::*;

pub mod database_environment;
pub mod logging_environment;

#[derive(Debug, serde::Deserialize)]
pub struct EnvironmentConfig {
  #[serde(flatten)]
  database_env: DatabaseEnvironment,

  #[serde(
    with = "logging_environment",
    default = "logging_environment::default_log_level"
  )]
  log_level: tracing::Level,
}

impl EnvironmentConfig {
  pub fn new() -> Result<Self, AppError> {
    envy::from_env::<Self>().map_err(Into::into)
  }

  pub fn database(&self) -> &DatabaseEnvironment {
    &self.database_env
  }

  pub fn log_level(&self) -> &tracing::Level {
    &self.log_level
  }
}
