#[derive(serde::Deserialize)]
pub struct DatabaseEnvironment {
  #[serde(default = "DatabaseEnvironment::default_database_username")]
  database_username: Option<String>,
  #[serde(default = "DatabaseEnvironment::default_database_password")]
  database_password: Option<String>,

  #[serde(default = "DatabaseEnvironment::default_database_address")]
  database_address: Option<String>,

  #[serde(default = "DatabaseEnvironment::default_database_name")]
  database_name: Option<String>,
}

impl std::fmt::Debug for DatabaseEnvironment {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{{ Database Environment Redacted }}")
  }
}

impl DatabaseEnvironment {
  const DEFAULT_DATABASE_USERNAME: &str = "root";
  const DEFAULT_DATABASE_PASSWORD: &str = "password";
  const DEFAULT_DATABASE_ADDRESS: &str = "127.0.0.1:3306";
  const DEFAULT_DATABASE_NAME: &str = "emptor";

  fn default_database_username() -> Option<String> {
    Some(Self::DEFAULT_DATABASE_USERNAME.to_string())
  }

  fn default_database_password() -> Option<String> {
    Some(Self::DEFAULT_DATABASE_PASSWORD.to_string())
  }

  fn default_database_address() -> Option<String> {
    Some(Self::DEFAULT_DATABASE_ADDRESS.to_string())
  }

  fn default_database_name() -> Option<String> {
    Some(Self::DEFAULT_DATABASE_NAME.to_string())
  }

  pub fn username(&self) -> Option<&str> {
    self.database_username.as_deref()
  }

  pub fn password(&self) -> Option<&str> {
    self.database_password.as_deref()
  }

  pub fn address(&self) -> Option<&str> {
    self.database_address.as_deref()
  }

  pub fn name(&self) -> Option<&str> {
    self.database_name.as_deref()
  }
}
