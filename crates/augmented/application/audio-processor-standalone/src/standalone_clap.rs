use std::ffi::CStr;

pub use clack_plugin;
use clack_plugin::extensions::PluginExtensions;
use clack_plugin::host::HostAudioThreadHandle;
use clack_plugin::plugin::descriptor::{PluginDescriptor, StaticPluginDescriptor};
use clack_plugin::plugin::{AudioConfiguration, Plugin, PluginError};
use clack_plugin::prelude::{Audio, Process, ProcessEvents, ProcessStatus};

use audio_processor_traits::{AudioContext, AudioProcessor, InterleavedAudioBuffer};

use crate::standalone_vst::{StandalonePluginContext, StandaloneProcessorFactory};
use crate::StandaloneProcessor;

pub struct StandaloneClackPlugin<SP> {
    processor: SP,
}

impl<'a, SP> Plugin<'a> for StandaloneClackPlugin<SP>
where
    SP: StandaloneProcessor,
    SP: StandaloneProcessorFactory<Output = SP>,
{
    type Shared = ();
    type MainThread = ();

    fn get_descriptor() -> Box<dyn PluginDescriptor> {
        use clack_plugin::plugin::descriptor::features::*;

        Box::new(StaticPluginDescriptor {
            id: CStr::from_bytes_with_nul(b"org.beijaflor.test\0").unwrap(),
            name: CStr::from_bytes_with_nul(b"Clack Standalone Example\0").unwrap(),
            features: Some(&[SYNTHESIZER, STEREO]),
            ..Default::default()
        })
    }

    fn activate(
        _host: HostAudioThreadHandle<'a>,
        _main_thread: &mut Self::MainThread,
        _shared: &'a Self::Shared,
        _audio_config: AudioConfiguration,
    ) -> Result<Self, PluginError> {
        let processor = SP::new_for_host(StandalonePluginContext {});
        Ok(Self { processor })
    }

    fn process(
        &mut self,
        _process: &Process,
        mut audio: Audio,
        _events: ProcessEvents,
    ) -> Result<ProcessStatus, PluginError> {
        let mut context = AudioContext::default();
        let mut output = audio.output(0).unwrap();
        let mut buffer = InterleavedAudioBuffer::new(
            0, // output.channel_count() as usize
            output
                .channels_mut()
                .as_f32_mut()
                .unwrap()
                .get_channel_data_mut(0)
                .unwrap(),
        );

        self.processor
            .processor()
            .process(&mut context, &mut buffer);

        Ok(ProcessStatus::Continue)
    }

    fn deactivate(self, _main_thread: &mut Self::MainThread) {}

    fn reset(&mut self, _main_thread: &mut Self::MainThread) {}

    fn start_processing(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    fn stop_processing(&mut self) {}

    fn declare_extensions(_builder: &mut PluginExtensions<Self>, _shared: &Self::Shared) {}
}

#[macro_export]
macro_rules! standalone_clap {
    ($t:ty) => {
        mod clack_impl {
            use audio_processor_standalone::standalone_clap::clack_plugin::prelude::*;
            use audio_processor_standalone::standalone_clap::*;
            use audio_processor_standalone::standalone_processor::*;

            use super::*;

            type StandaloneProcessorImpl = StandaloneAudioOnlyProcessor<$t>;

            #[allow(non_upper_case_globals)]
            #[allow(unsafe_code)]
            #[no_mangle]
            pub static clap_entry: PluginEntryDescriptor =
                SinglePluginEntry::<StandaloneClackPlugin<StandaloneProcessorImpl>>::DESCRIPTOR;
        }

        pub use clack_impl::*;
    };
}
