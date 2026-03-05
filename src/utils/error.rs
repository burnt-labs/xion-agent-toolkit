use thiserror::Error;

#[derive(Debug, Error)]
pub enum XionError {
    #[error("Configuration error: {0}")]
    Config(#[from] anyhow::Error),

    #[error("OAuth2 error: {0}")]
    OAuth2(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, XionError>;
