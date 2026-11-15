use crate::env_config::EnvironmentConfig;
use crate::error::AppError;
use crate::inputs::terminal_config::TerminalConfig;
use crate::logging::initialize_logging;
use crossterm::event::{KeyCode, KeyModifiers};
use tracing_appender::non_blocking::WorkerGuard;

pub struct AppConfig {
  _logging_guard: WorkerGuard,
  environment_config: EnvironmentConfig,
  terminal_config: TerminalConfig,
}

impl AppConfig {
  pub fn new() -> Result<Self, AppError> {
    let environment_config = EnvironmentConfig::new()?;
    let _logging_guard = initialize_logging(&environment_config);
    let terminal_config = TerminalConfig::new()?;

    TerminalConfig::setup()?;

    Ok(Self {
      _logging_guard,
      environment_config,
      terminal_config,
    })
  }

  pub fn environment_config(&self) -> &EnvironmentConfig {
    &self.environment_config
  }

  pub fn terminal_config(&self) -> &TerminalConfig {
    &self.terminal_config
  }

  pub fn mut_terminal_config(&mut self) -> &mut TerminalConfig {
    &mut self.terminal_config
  }

  pub async fn run(mut self) -> Result<(), AppError> {
    let mut buffer = String::new();

    loop {
      let Some(key_event) = TerminalConfig::get_input()? else {
        tracing::info!("No input detected.");
        continue;
      };

      match key_event.code {
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
          tracing::info!("Breaking main loop");

          break;
        }
        KeyCode::Char(character) => {
          buffer.push(character);
        }
        KeyCode::Backspace => {
          buffer.pop();
        }

        KeyCode::Enter => {
          break;
        }

        _ => {
          tracing::warn!("Known input {key_event:?}");
        }
      }
    }

    tracing::info!("Final buffer: {buffer}");

    todo!("App config runner")
  }
}
