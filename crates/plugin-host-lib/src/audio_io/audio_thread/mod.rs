use std::any::Any;
use std::error::Error;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, StreamConfig};
use thiserror::Error;

use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, NoopAudioProcessor};

use crate::processors::audio_file_processor::AudioFileError;

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
    #[error("Audio thread isn't running")]
    NotStartedError,
    #[error("Unknown error")]
    UnknownError(Box<dyn Any + Send>),
}

pub struct AudioThread {
    processor: Arc<AtomicPtr<Box<dyn AudioProcessor>>>,
    handle: Option<JoinHandle<Result<(), AudioThreadError>>>,
}

impl AudioThread {}

impl AudioThread {
    pub fn new() -> Self {
        let processor = NoopAudioProcessor;
        let processor: Box<dyn AudioProcessor> = Box::new(processor);
        let processor = Box::into_raw(Box::new(processor));
        AudioThread {
            processor: Arc::new(AtomicPtr::new(processor)),
            handle: None,
        }
    }

    pub fn start(&mut self) {
        let processor = self.processor.clone();
        self.handle = Some(thread::spawn(move || -> Result<(), AudioThreadError> {
            initialize_audio_thread(processor)
        }))
    }

    pub fn wait(mut self) -> Result<(), AudioThreadError> {
        let handle = self.handle.ok_or(AudioThreadError::NotStartedError)?;
        handle
            .join()
            .map_err(|err| AudioThreadError::UnknownError(err))?;
        Ok(())
    }

    pub fn set_processor(&mut self, processor: Box<dyn AudioProcessor>) {
        log::info!("Updating audio processor");
        let new_processor_ptr = Box::into_raw(Box::new(processor));
        let old_processor_ptr = self.processor.swap(new_processor_ptr, Ordering::Relaxed);
        unsafe {
            // Let the old processor be dropped
            let _old_processor_ptr = Box::from_raw(old_processor_ptr);
        }
    }

    pub fn settings() -> Result<AudioProcessorSettings, AudioThreadError> {
        // TODO - This should be queried from the audio thread.
        let cpal_host = cpal::default_host();
        let output_device = cpal_host
            .default_output_device()
            .ok_or(AudioThreadError::OutputDeviceNotFoundError)?;
        let output_config = output_device.default_output_config()?;
        let output_config: StreamConfig = output_config.into();
        let channels = output_config.channels as usize;
        let audio_settings = AudioProcessorSettings::new(
            output_config.sample_rate.0 as f32,
            channels,
            channels,
            512,
        );

        Ok(audio_settings)
    }
}

fn initialize_audio_thread(
    processor: Arc<AtomicPtr<Box<dyn AudioProcessor>>>,
) -> Result<(), AudioThreadError> {
    let cpal_host = cpal::default_host();
    log::info!("Using host: {}", cpal_host.id().name());
    let output_device = cpal_host
        .default_output_device()
        .ok_or(AudioThreadError::OutputDeviceNotFoundError)?;
    log::info!("Using device: {}", output_device.name()?);
    let output_config = output_device.default_output_config()?;
    let sample_format = output_config.sample_format();
    let mut output_config: StreamConfig = output_config.into();
    output_config.buffer_size = BufferSize::Fixed(512);

    match sample_format {
        SampleFormat::F32 => unsafe {
            run_main_loop(processor, &output_device, &output_config)?;
            Ok(())
        },
        _ => Err(AudioThreadError::UnsupportedSampleFormat),
    }
}

unsafe fn run_main_loop(
    processor: Arc<AtomicPtr<Box<dyn AudioProcessor>>>,
    output_device: &cpal::Device,
    output_config: &cpal::StreamConfig,
) -> Result<(), AudioThreadError> {
    let buffer_size = match output_config.buffer_size {
        BufferSize::Default => Err(AudioThreadError::UnexpectedDefaultBufferSize),
        BufferSize::Fixed(buffer_size) => Ok(buffer_size),
    }?;

    let sample_rate = output_config.sample_rate.0 as f32;
    let channels = output_config.channels as usize;

    log::info!("Buffer size {:?}", buffer_size);
    let audio_settings = AudioProcessorSettings::new(sample_rate, channels, channels, buffer_size);
    {
        let processor = processor.load(Ordering::Relaxed);
        (*processor).prepare(audio_settings);
    }

    let stream = output_device.build_output_stream(
        output_config,
        move |data: &mut [f32], _output_info: &cpal::OutputCallbackInfo| {
            let processor = processor.load(Ordering::Relaxed);
            (*processor).process(data);
        },
        |err| {
            log::error!("Playback error: {:?}", err);
        },
    )?;

    stream.play()?;

    std::thread::park();

    Ok(())
}
