#[derive(Debug, thiserror::Error)]
pub enum AppError {
  #[error(transparent)]
  Envy(#[from] envy::Error),

  #[error(transparent)]
  Io(#[from] std::io::Error),

  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),

  #[error(transparent)]
  DbErr(#[from] sea_orm::DbErr),

  #[error("Failed to build a database URL from missing values: `{missing:?}`")]
  MissingDatabaseEnvironment { missing: Vec<&'static str> },
}
