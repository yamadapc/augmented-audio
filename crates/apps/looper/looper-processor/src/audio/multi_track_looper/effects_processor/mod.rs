use basedrop::{Shared, SharedCell};
use std::ops::Deref;

use audio_garbage_collector::{make_shared, make_shared_cell};
use audio_processor_bitcrusher::{BitCrusherProcessor};
use audio_processor_graph::{AudioProcessorGraph, AudioProcessorGraphHandle, NodeType};
use audio_processor_time::MonoDelayProcessor;
use audio_processor_time::{FreeverbProcessor};
use audio_processor_traits::parameters::{
    AudioProcessorHandleProvider, AudioProcessorHandleRef,
};
use audio_processor_traits::{
    simple_processor, AudioBuffer, AudioProcessor, AudioProcessorSettings, SliceAudioProcessor,
};
use augmented_dsp_filters::rbj::{FilterProcessor, FilterType};

enum EffectType {
    Reverb,
    Delay,
    Filter,
    BitCrusher,
}

struct EffectsProcessorHandle {
    graph_handle: Shared<AudioProcessorGraphHandle>,
    effect_handles: SharedCell<Vec<AudioProcessorHandleRef>>,
    settings: SharedCell<AudioProcessorSettings>,
}

impl EffectsProcessorHandle {
    pub fn add_effect(&self, effect: EffectType) {
        let (processor, handle): (
            Box<dyn SliceAudioProcessor + Send + 'static>,
            AudioProcessorHandleRef,
        ) = {
            use EffectType::*;

            let mut handle: Option<AudioProcessorHandleRef> = None;
            let effect: Box<dyn SliceAudioProcessor + Send + 'static> = match effect {
                Reverb => {
                    let processor = FreeverbProcessor::default();
                    handle = Some(processor.generic_handle());
                    Box::new(simple_processor::BufferProcessor(processor))
                }
                Delay => {
                    let mono_delay_processor = MonoDelayProcessor::default();
                    handle = Some(mono_delay_processor.generic_handle());
                    Box::new(simple_processor::BufferProcessor(mono_delay_processor))
                }
                Filter => {
                    let processor = FilterProcessor::new(FilterType::LowPass);
                    handle = Some(processor.generic_handle());
                    Box::new(simple_processor::BufferProcessor(processor))
                }
                BitCrusher => {
                    let processor = BitCrusherProcessor::default();
                    handle = Some(AudioProcessorHandleProvider::generic_handle(&processor));
                    Box::new(processor)
                }
            };

            let handle = handle.unwrap();
            (effect, handle)
        };

        let node_idx = self.graph_handle.add_node(NodeType::Buffer(processor));
        let _ = self
            .graph_handle
            .add_connection(self.graph_handle.input(), node_idx);
        let _ = self
            .graph_handle
            .add_connection(node_idx, self.graph_handle.output());

        let mut handles_vec: Vec<AudioProcessorHandleRef> =
            (*self.effect_handles.get().deref()).clone();
        handles_vec.push(handle);
        self.effect_handles.set(make_shared(handles_vec));
    }
}

struct EffectsProcessor {
    handle: Shared<EffectsProcessorHandle>,
    graph: AudioProcessorGraph,
}

impl EffectsProcessor {
    pub fn new() -> Self {
        let graph = AudioProcessorGraph::default();
        let graph_handle = graph.handle().clone();

        Self {
            graph,
            handle: make_shared(EffectsProcessorHandle {
                graph_handle,
                effect_handles: make_shared_cell(vec![]),
                settings: make_shared_cell(Default::default()),
            }),
        }
    }
}

impl AudioProcessor for EffectsProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.graph.prepare(settings);
        self.handle.settings.set(make_shared(settings));
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        self.graph.process(data)
    }
}
