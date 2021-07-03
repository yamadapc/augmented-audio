use basedrop::{Handle, Shared, SharedCell};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;

use audio_processor_traits::InterleavedAudioBuffer;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, SilenceAudioProcessor};
use error::AudioThreadError;
use options::AudioThreadOptions;

use crate::audio_io::audio_thread::options::AudioDeviceId;
use crate::processors::shared_processor::{ProcessorCell, SharedProcessor};
use crate::processors::test_host_processor::TestHostProcessor;

mod cpal_option_handling;
pub mod error;
pub mod options;

pub enum AudioThreadProcessor {
    Active(TestHostProcessor),
    Silence(SilenceAudioProcessor),
}

pub struct AudioThread {
    processor: Shared<SharedCell<ProcessorCell<AudioThreadProcessor>>>,
    processor_ref: SharedProcessor<AudioThreadProcessor>,
    stream: Option<cpal::Stream>,
    audio_thread_options: AudioThreadOptions,
}

unsafe impl Send for AudioThread {}

impl AudioThread {
    pub fn new(handle: &Handle) -> Self {
        let processor = AudioThreadProcessor::Silence(SilenceAudioProcessor);
        let processor_ref = SharedProcessor::new(handle, processor);
        AudioThread {
            processor: Shared::new(handle, SharedCell::new(processor_ref.shared())),
            processor_ref,
            stream: None,
            audio_thread_options: AudioThreadOptions::default(),
        }
    }

    pub fn start(&mut self) -> Result<(), AudioThreadError> {
        let processor = self.processor.clone();
        let audio_thread_options = self.audio_thread_options.clone();
        let stream = create_stream(&audio_thread_options, processor)?;
        log::info!("Starting CPAL output stream");
        stream.play()?;
        self.stream = Some(stream);
        Ok(())
    }

    /// Change output device & restart audio thread
    pub fn set_output_device_id(
        &mut self,
        output_device_id: AudioDeviceId,
    ) -> Result<(), AudioThreadError> {
        if output_device_id != self.audio_thread_options.output_device_id {
            self.audio_thread_options.output_device_id = output_device_id;
            self.wait()?;
            self.start()?;
        }
        Ok(())
    }

    pub fn wait(&self) -> Result<(), AudioThreadError> {
        if let Some(stream) = &self.stream {
            stream.pause()?;
        }
        Ok(())
    }

    /// # Safety:
    /// The processor MUST be prepared for playback when it's set.
    pub fn set_processor(&mut self, processor: SharedProcessor<AudioThreadProcessor>) {
        log::info!("Updating audio processor");
        self.processor_ref = processor.clone();
        let _old_processor_ptr = self.processor.replace(processor.shared());
        // Let the old processor be dropped
    }

    pub fn default_settings() -> Result<AudioProcessorSettings, AudioThreadError> {
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

fn create_stream(
    options: &AudioThreadOptions,
    processor: Shared<SharedCell<ProcessorCell<AudioThreadProcessor>>>,
) -> Result<cpal::Stream, AudioThreadError> {
    let host = cpal_option_handling::get_cpal_host(&options.host_id);
    let output_device =
        cpal_option_handling::get_cpal_output_device(&host, &options.output_device_id)?;
    log::info!("Using device: {}", output_device.name()?);
    let output_config = cpal_option_handling::get_output_config(&options, &output_device)?;
    let stream = create_stream_inner(processor, &output_device, &output_config)?;
    Ok(stream)
}

fn create_stream_inner(
    processor: Shared<SharedCell<ProcessorCell<AudioThreadProcessor>>>,
    output_device: &cpal::Device,
    output_config: &cpal::StreamConfig,
) -> Result<cpal::Stream, AudioThreadError> {
    let buffer_size = match output_config.buffer_size {
        cpal::BufferSize::Default => Err(AudioThreadError::UnexpectedDefaultBufferSize),
        cpal::BufferSize::Fixed(buffer_size) => Ok(buffer_size),
    }?;

    log::info!("Buffer size {:?}", buffer_size);

    let num_channels: usize = output_config.channels.into();
    Ok(output_device.build_output_stream(
        output_config,
        move |data: &mut [f32], _output_info: &cpal::OutputCallbackInfo| unsafe {
            let mut audio_buffer = InterleavedAudioBuffer::new(num_channels, data);

            let shared_processor = processor.get();
            let processor_ptr = shared_processor.0.get();
            match &mut (*processor_ptr) {
                AudioThreadProcessor::Active(processor) => processor.process(&mut audio_buffer),
                AudioThreadProcessor::Silence(processor) => (*processor).process(&mut audio_buffer),
            }
        },
        |err| {
            log::error!("Playback error: {:?}", err);
        },
    )?)
}
