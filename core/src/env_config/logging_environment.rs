use serde::{Deserialize, Deserializer, Serializer};

pub const DEFAULT_LOG_LEVEL: tracing::Level = tracing::Level::DEBUG;

pub fn default_log_level() -> tracing::Level {
  DEFAULT_LOG_LEVEL
}

pub fn serialize<S>(level: &tracing::Level, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(level.as_str())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<tracing::Level, D::Error>
where
  D: Deserializer<'de>,
{
  String::deserialize(deserializer)?
    .parse()
    .map_err(serde::de::Error::custom)
}
