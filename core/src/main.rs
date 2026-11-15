use core::app_config::AppConfig;

#[tokio::main]
async fn main() {
  let app_config = match AppConfig::new() {
    Ok(app_config) => app_config,
    Err(error) => {
      println!("Failed to build app config. Reason: `{error}`");

      std::process::exit(1)
    }
  };

  if let Err(error) = tokio::spawn(app_config.run()).await {
    println!("Fatal error when running app. Reason: {error}");

    std::process::exit(1);
  }

  std::process::exit(0);
}
