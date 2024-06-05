use cpal::{SampleFormat, HostId};
use thiserror::Error;
/// The `actually_beep` Error type
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported sample format '{0}'")]
    UnsupportedSampleFormat(SampleFormat),
    #[error("cpal error: {0}")]
    CpalError(Box<dyn std::error::Error>),
    #[error("An error occurred on stream: {0}")]
    StreamError(cpal::StreamError),
    #[error("Failed to find output device \"{0}\"")]
    NoDevice(String),
    #[error("Host not found: {0:?}")]
    HostNotFound(HostId)
}
/// The `actually_beep` Result type
pub type Result<S> = std::result::Result<S, Error>;
