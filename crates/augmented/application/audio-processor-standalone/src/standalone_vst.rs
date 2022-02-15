use std::marker::PhantomData;
use std::sync::Arc;
pub use vst;
use vst::plugin::PluginParameters;
use vst::{
    buffer::AudioBuffer as VSTAudioBuffer,
    plugin::{HostCallback, Info},
};

use audio_processor_traits::{
    audio_buffer::OwnedAudioBuffer, AudioBuffer, AudioProcessor, AudioProcessorSettings,
    VecAudioBuffer,
};

use crate::{StandaloneAudioOnlyProcessor, StandaloneProcessor, StandaloneProcessorImpl};

/// TODO - Extend this with VST helpers calling into the host callback.
pub struct StandalonePluginContext {}

pub trait StandaloneProcessorFactory {
    type Output;

    fn new_for_host(context: StandalonePluginContext) -> Self::Output;
}

impl<D: Default> StandaloneProcessorFactory for D {
    type Output = Self;

    fn new_for_host(_context: StandalonePluginContext) -> Self {
        Self::default()
    }
}

impl<P> StandaloneProcessorFactory for StandaloneProcessorImpl<P>
where
    P: StandaloneProcessorFactory<Output = P> + AudioProcessor<SampleType = f32>,
{
    type Output = Self;

    fn new_for_host(context: StandalonePluginContext) -> Self::Output {
        StandaloneProcessorImpl::new(P::new_for_host(context))
    }
}

impl<P> StandaloneProcessorFactory for StandaloneAudioOnlyProcessor<P>
where
    P: StandaloneProcessorFactory<Output = P> + AudioProcessor<SampleType = f32>,
{
    type Output = Self;

    fn new_for_host(context: StandalonePluginContext) -> Self {
        StandaloneAudioOnlyProcessor::new(P::new_for_host(context), Default::default())
    }
}

pub struct StandaloneVSTPlugin<SPF, SP> {
    processor: SP,
    buffer: VecAudioBuffer<f32>,
    settings: AudioProcessorSettings,
    factory: PhantomData<SPF>,
}

#[macro_export]
macro_rules! standalone_vst {
    ($t:ty) => {
        ::audio_processor_standalone::standalone_vst::vst::plugin_main!(
            ::audio_processor_standalone::standalone_vst::StandaloneVSTPlugin<
                ::audio_processor_standalone::standalone_processor::StandaloneAudioOnlyProcessor<
                    $t,
                >,
                ::audio_processor_standalone::standalone_processor::StandaloneAudioOnlyProcessor<
                    $t,
                >,
            >
        );
    };
    ($t:ty, $f: ty) => {
        ::audio_processor_standalone::standalone_vst::vst::plugin_main!(
            ::audio_processor_standalone::standalone_vst::StandaloneVSTPlugin<
                $f,
                ::audio_processor_standalone::standalone_processor::StandaloneAudioOnlyProcessor<
                    $t,
                >,
            >
        );
    };
}

#[macro_export]
macro_rules! generic_standalone_vst {
    ($t: ty) => {
        struct StandaloneFactory {}
        impl ::audio_processor_standalone::standalone_vst::StandaloneProcessorFactory for StandaloneFactory {
            type Output = ::audio_processor_standalone::standalone_processor::StandaloneAudioOnlyProcessor<$t>;

            fn new_for_host(
                _context: ::audio_processor_standalone::standalone_vst::StandalonePluginContext
            ) -> Self::Output {
                let processor = <$t>::default();
                let options = ::audio_processor_standalone::standalone_processor::StandaloneOptions {
                    handle: Some(::std::sync::Arc::new(processor.generic_handle())),
                    ..Default::default()
                };
                ::audio_processor_standalone::standalone_processor::StandaloneAudioOnlyProcessor::new(processor, options)
            }
        }

        ::audio_processor_standalone::standalone_vst!($t, StandaloneFactory);
    }
}

impl<ProcessorFactory, Processor> vst::plugin::Plugin
    for StandaloneVSTPlugin<ProcessorFactory, Processor>
where
    ProcessorFactory: StandaloneProcessorFactory<Output = Processor> + Send,
    Processor: StandaloneProcessor,
    <Processor as StandaloneProcessor>::Processor: AudioProcessor<SampleType = f32>,
{
    fn get_info(&self) -> Info {
        Info { ..Info::default() }
    }

    fn new(_host: HostCallback) -> Self
    where
        Self: Sized,
    {
        Self {
            processor: ProcessorFactory::new_for_host(StandalonePluginContext {}),
            buffer: VecAudioBuffer::new(),
            settings: AudioProcessorSettings::default(),
            factory: PhantomData::default(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.settings.sample_rate = rate;
        self.processor.processor().prepare(self.settings);
    }

    fn set_block_size(&mut self, size: i64) {
        self.buffer.resize(2, size as usize, 0.0);
        self.settings.block_size = size as usize;
        self.processor.processor().prepare(self.settings);
    }

    fn resume(&mut self) {
        self.processor.processor().prepare(self.settings);
    }

    fn process(&mut self, buffer: &mut VSTAudioBuffer<f32>) {
        let num_samples = buffer.samples();
        let (inputs, mut outputs) = buffer.split();

        self.buffer.resize(2, num_samples as usize, 0.0);
        {
            let buffer_slice = self.buffer.slice_mut();
            for (channel, input) in inputs.into_iter().take(2).enumerate() {
                for (index, sample) in input.iter().enumerate() {
                    buffer_slice[index * 2 + channel] = *sample;
                }
            }
        }

        self.processor.processor().process(&mut self.buffer);

        {
            let buffer_slice = self.buffer.slice();
            for (channel, output) in outputs.into_iter().take(2).enumerate() {
                for (index, sample) in output.iter_mut().enumerate() {
                    *sample = buffer_slice[index * 2 + channel];
                }
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::new(DummyPluginParameters)
    }

    #[cfg(feature = "gui")]
    fn get_editor(&mut self) -> Option<Box<dyn vst::editor::Editor>> {
        self.processor
            .handle()
            .map(|handle| crate::gui::editor(handle))
    }
}

struct DummyPluginParameters;

impl PluginParameters for DummyPluginParameters {}
