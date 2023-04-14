use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioFileError {
    #[error("Failed to decode input file")]
    DecodeError(#[from] symphonia::core::errors::Error),
    #[error("Failed to read input file")]
    FileReadError(#[from] std::io::Error),
    #[error("Failed to open read stream")]
    OpenStreamError,
    #[error("File has no buffers")]
    EmptyFileError,
}
