// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::marker::PhantomData;
use std::sync::Arc;

pub use vst;
use vst::plugin::PluginParameters;
use vst::{
    buffer::AudioBuffer as VSTAudioBuffer,
    plugin::{HostCallback, Info},
};

use audio_processor_traits::audio_buffer::vst::VSTBufferHandler;
use audio_processor_traits::{AudioContext, AudioProcessor, AudioProcessorSettings};

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
    buffer_handler: VSTBufferHandler<f32>,
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
                    handle: Some(::audio_processor_traits::parameters::AudioProcessorHandleProvider::generic_handle(&processor)),
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
            buffer_handler: VSTBufferHandler::new(),
            settings: AudioProcessorSettings::default(),
            factory: PhantomData::default(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.settings.sample_rate = rate;
        let mut context = AudioContext::from(self.settings);
        self.processor
            .processor()
            .prepare(&mut context, self.settings);
    }

    fn set_block_size(&mut self, size: i64) {
        self.buffer_handler.set_block_size(size as usize);
        self.settings.block_size = size as usize;
        let mut context = AudioContext::from(self.settings);
        self.processor
            .processor()
            .prepare(&mut context, self.settings);
    }

    fn resume(&mut self) {
        let mut context = AudioContext::from(self.settings);
        self.processor
            .processor()
            .prepare(&mut context, self.settings);
    }

    fn process(&mut self, vst_buffer: &mut VSTAudioBuffer<f32>) {
        let processor = self.processor.processor();

        let mut context = AudioContext::from(self.settings);
        self.buffer_handler.with_buffer(vst_buffer, |buffer| {
            processor.process(&mut context, buffer);
        });
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::new(DummyPluginParameters)
    }

    #[cfg(feature = "gui")]
    fn get_editor(&mut self) -> Option<Box<dyn ::vst::editor::Editor>> {
        self.processor
            .handle()
            .map(::audio_processor_standalone_gui::editor)
    }
}

struct DummyPluginParameters;

impl PluginParameters for DummyPluginParameters {}
