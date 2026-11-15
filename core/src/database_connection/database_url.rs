use crate::{env_config::database_environment::DatabaseEnvironment, error::AppError};

pub struct DatabaseUrl(String);

impl DatabaseUrl {
  pub fn new(database_environment: &DatabaseEnvironment) -> Result<Self, AppError> {
    Self::check_environment(database_environment)?;

    let username = database_environment.username().unwrap();
    let password = database_environment.password().unwrap();
    let address = database_environment.address().unwrap();
    let database_name = database_environment.name().unwrap();

    let url = Self(format!(
      "mysql://{username}:{password}&{address}/{database_name}"
    ));

    Ok(url)
  }

  pub fn get(&self) -> &str {
    &self.0
  }

  fn check_environment(database_environment: &DatabaseEnvironment) -> Result<(), AppError> {
    let username = database_environment.username();
    let password = database_environment.password();
    let address = database_environment.address();
    let database_name = database_environment.name();

    let errors = {
      let mut errors = vec![
        username.map_or_else(|| "username", |_| ""),
        password.map_or_else(|| "password", |_| ""),
        address.map_or_else(|| "address", |_| ""),
        database_name.map_or_else(|| "database_name", |_| ""),
      ];
      errors.retain(|error| !error.is_empty());

      errors
    };

    if !errors.is_empty() {
      return Err(AppError::MissingDatabaseEnvironment { missing: errors });
    }

    Ok(())
  }
}
