use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse config file {0}")]
    ConfigError(#[from] toml::de::Error),

    #[error("Failed to open file {0}")]
    IoError(#[from] std::io::Error),

    #[error("Subsonic api error {0}")]
    SubApiError(#[from] sunk::Error),

    #[error("Streaming error {0}")]
    StreamError(#[from] rodio::StreamError),

    #[error("Playing error {0}")]
    PlayError(#[from] rodio::PlayError),

    #[error("Decoder error {0}")]
    DecoderError(#[from] rodio::decoder::DecoderError),
}
