use crate::{
  database_connection::database_url::DatabaseUrl, env_config::EnvironmentConfig, error::AppError,
};
use sea_orm::*;
use std::sync::Arc;

const ENTITIES_PATH: &str = "entities::*";

pub struct DatabasePool {
  database_connection: Arc<DatabaseConnection>,
}

impl DatabasePool {
  pub async fn new(environment_config: &EnvironmentConfig) -> Result<Self, AppError> {
    let database_connection = Self::create_connection(environment_config).await?;

    Ok(Self {
      database_connection: Arc::new(database_connection),
    })
  }

  pub fn get_connection(&self) -> &DatabaseConnection {
    &self.database_connection
  }

  async fn create_connection(
    environment_config: &EnvironmentConfig,
  ) -> Result<DatabaseConnection, AppError> {
    let database_environment = environment_config.database();
    let database_url = DatabaseUrl::new(database_environment)?;

    let mut opt = ConnectOptions::new(database_url.get());
    opt.max_connections(50);

    let database_connection = Database::connect(opt).await?;

    database_connection
      .get_schema_registry(ENTITIES_PATH)
      .sync(&database_connection)
      .await?;

    Ok(database_connection)
  }
}
