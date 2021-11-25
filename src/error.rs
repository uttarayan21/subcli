use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse config file")]
    ConfigError(#[from] toml::de::Error),

    #[error("Failed to open file")]
    IoError(#[from] std::io::Error),

    #[error("SomeError")]
    SomeError,
}
