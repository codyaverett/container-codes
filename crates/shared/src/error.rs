use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Container error: {0}")]
    Container(String),

    #[error("Job error: {0}")]
    Job(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing configuration value: {key}")]
    Missing { key: String },

    #[error("Invalid configuration value for {key}: {value}")]
    Invalid { key: String, value: String },

    #[error("Configuration parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("File not found: {path}")]
    FileNotFound { path: String },
}

impl Error {
    pub fn config_missing(key: impl Into<String>) -> Self {
        Self::Config(ConfigError::Missing { key: key.into() })
    }

    pub fn config_invalid(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Config(ConfigError::Invalid {
            key: key.into(),
            value: value.into(),
        })
    }

    pub fn http(msg: impl Into<String>) -> Self {
        Self::Http(msg.into())
    }

    pub fn container(msg: impl Into<String>) -> Self {
        Self::Container(msg.into())
    }

    pub fn job(msg: impl Into<String>) -> Self {
        Self::Job(msg.into())
    }

    pub fn auth(msg: impl Into<String>) -> Self {
        Self::Auth(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}