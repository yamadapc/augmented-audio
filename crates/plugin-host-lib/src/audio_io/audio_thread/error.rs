use std::any::Any;

use thiserror::Error;

use crate::processors::audio_file_processor::file_io::AudioFileError;

#[derive(Error, Debug)]
pub enum AudioThreadError {
    #[error("Unsupported sample format from device.")]
    UnsupportedSampleFormat,
    #[error("Failed to get audio device name")]
    DeviceNameError(#[from] cpal::DeviceNameError),
    #[error("Failed to read input file")]
    InputFileError(#[from] AudioFileError),
    #[error("Failed to get assigned or default audio device")]
    OutputDeviceNotFoundError,
    #[error("Failed to get default output stream configuration")]
    DefaultStreamConfigError(#[from] cpal::DefaultStreamConfigError),
    #[error("Buffer size needs to be set to a fixed value")]
    UnexpectedDefaultBufferSize,
    #[error("Failed to build output stream")]
    BuildStreamError(#[from] cpal::BuildStreamError),
    #[error("Failed to start playback")]
    PlayStreamError(#[from] cpal::PlayStreamError),
    #[error("Failed to pause playback")]
    PauseStreamError(#[from] cpal::PauseStreamError),
    #[error("Unknown error")]
    UnknownError(Box<dyn Any + Send>),
}
