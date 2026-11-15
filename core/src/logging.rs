use crate::env_config::EnvironmentConfig;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

const LOGGING_DIRECTORY: &str = "logs";
const FILE_NAME_PREFIX: &str = "app.log";

pub fn initialize_logging(envionment_config: &EnvironmentConfig) -> WorkerGuard {
  let log_level = envionment_config.log_level();
  let file_appender = rolling::daily(LOGGING_DIRECTORY, FILE_NAME_PREFIX);
  let (non_blocking_writer, guard) = non_blocking(file_appender);

  let file_layer = fmt::layer()
    .with_writer(non_blocking_writer)
    .with_ansi(false);

  let stdout_layer = fmt::layer();

  Registry::default()
    .with(EnvFilter::new(log_level.to_string()))
    .with(file_layer)
    .with(stdout_layer)
    .init();

  guard
}
