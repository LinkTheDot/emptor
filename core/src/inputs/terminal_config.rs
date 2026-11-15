use crate::error::AppError;
use crossterm::{
  cursor,
  event::{Event, KeyEvent},
  execute, terminal,
};
use std::io::stdout;
use terminal_display::buffer::*;

pub struct TerminalConfig {
  display: TerminalDisplay,
}

impl TerminalConfig {
  pub fn new() -> Result<Self, AppError> {
    let terminal_dimensions = TerminalDisplay::terminal_dimensions()?;
    let display = TerminalDisplay::new(terminal_dimensions.0, terminal_dimensions.1, 0.5, 0.5)?;

    Ok(Self { display })
  }

  pub fn setup() -> Result<(), AppError> {
    execute!(stdout(), cursor::Hide, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    Ok(())
  }

  /// Returns the [`KeyEvent`](crossterm::event::KeyEvent) buffer from the user.
  pub fn get_input() -> Result<Option<KeyEvent>, AppError> {
    if let Event::Key(key_event) = crossterm::event::read()? {
      Ok(Some(key_event))
    } else {
      Ok(None)
    }
  }

  pub fn display(&self) -> &TerminalDisplay {
    &self.display
  }

  pub fn mut_display(&mut self) -> &mut TerminalDisplay {
    &mut self.display
  }
}

impl Drop for TerminalConfig {
  fn drop(&mut self) {
    terminal::disable_raw_mode().unwrap();
    execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen).unwrap();
  }
}
