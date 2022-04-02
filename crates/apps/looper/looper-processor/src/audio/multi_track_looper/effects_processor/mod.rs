use basedrop::{Shared, SharedCell};
use std::ops::Deref;

use audio_garbage_collector::{make_shared, make_shared_cell};
use audio_processor_bitcrusher::BitCrusherProcessor;
use audio_processor_graph::{AudioProcessorGraph, AudioProcessorGraphHandle, NodeIndex, NodeType};
use audio_processor_time::FreeverbProcessor;
use audio_processor_time::MonoDelayProcessor;
use audio_processor_traits::parameters::{AudioProcessorHandleProvider, AudioProcessorHandleRef};
use audio_processor_traits::{
    simple_processor, AudioBuffer, AudioProcessor, AudioProcessorSettings, SliceAudioProcessor,
};
use augmented_dsp_filters::rbj::{FilterProcessor, FilterType};

type SomeEffectProcessor = Box<dyn SliceAudioProcessor + Send + 'static>;
type SomeHandle = AudioProcessorHandleRef;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum EffectType {
    EffectTypeReverb = 0,
    EffectTypeDelay = 1,
    EffectTypeFilter = 2,
    EffectTypeBitCrusher = 3,
}

#[derive(Clone)]
pub struct EffectNodeState {
    node_index: NodeIndex,
    handle: AudioProcessorHandleRef,
}

pub struct EffectsProcessorHandle {
    graph_handle: Shared<AudioProcessorGraphHandle>,
    effects: SharedCell<Vec<EffectNodeState>>,
    settings: SharedCell<AudioProcessorSettings>,
}

impl EffectsProcessorHandle {
    pub fn add_effect(&self, effect: EffectType) {
        let (mut processor, handle): (SomeEffectProcessor, AudioProcessorHandleRef) = {
            use EffectType::*;

            let (effect, handle): (SomeEffectProcessor, SomeHandle) = match effect {
                EffectTypeReverb => {
                    let processor = FreeverbProcessor::default();
                    let handle = processor.generic_handle();
                    (
                        Box::new(simple_processor::BufferProcessor(processor)),
                        handle,
                    )
                }
                EffectTypeDelay => {
                    let mono_delay_processor = MonoDelayProcessor::default();
                    let handle = mono_delay_processor.generic_handle();
                    (
                        Box::new(simple_processor::BufferProcessor(mono_delay_processor)),
                        handle,
                    )
                }
                EffectTypeFilter => {
                    let processor = FilterProcessor::new(FilterType::LowPass);
                    let handle = processor.generic_handle();
                    (
                        Box::new(simple_processor::BufferProcessor(processor)),
                        handle,
                    )
                }
                EffectTypeBitCrusher => {
                    let processor = BitCrusherProcessor::default();
                    processor.handle().set_sample_rate(100.0);
                    let handle = AudioProcessorHandleProvider::generic_handle(&processor);
                    (Box::new(processor), handle)
                }
            };

            let handle = handle;
            (effect, handle)
        };

        let settings = self.settings.get().deref().clone();
        processor.prepare_slice(settings);
        let node_idx = self.graph_handle.add_node(NodeType::Buffer(processor));
        let state = EffectNodeState {
            handle,
            node_index: node_idx,
        };
        let mut effects: Vec<EffectNodeState> = (*self.effects.get().deref()).clone();
        effects.push(state);
        self.effects.set(make_shared(effects));
        self.update_graph();
    }

    fn update_graph(&self) {
        // Ideally all of this should happen at once & switch the graph up. We also didn't need to
        // rebuild the whole graph.
        self.graph_handle.clear();
        log::info!("Rebuilding effects graph");
        let effects = self.effects.get();

        let mut last_index = self.graph_handle.input();
        for effect in &*effects {
            log::info!(
                "Connecting effect {:?} => {:?}",
                last_index,
                effect.node_index
            );
            let _ = self
                .graph_handle
                .add_connection(last_index, effect.node_index);
            last_index = effect.node_index;
        }

        let _ = self
            .graph_handle
            .add_connection(last_index, self.graph_handle.output());
    }
}

pub struct EffectsProcessor {
    handle: Shared<EffectsProcessorHandle>,
    graph: AudioProcessorGraph,
}

impl EffectsProcessor {
    pub fn new() -> Self {
        let mut graph = AudioProcessorGraph::default();
        let graph_handle = graph.handle().clone();

        graph
            .add_connection(graph.input(), graph.output())
            .expect("Can't cycle");

        Self {
            graph,
            handle: make_shared(EffectsProcessorHandle {
                graph_handle,
                effects: make_shared_cell(vec![]),
                settings: make_shared_cell(Default::default()),
            }),
        }
    }

    pub fn handle(&self) -> &Shared<EffectsProcessorHandle> {
        &self.handle
    }
}

impl AudioProcessor for EffectsProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        log::info!("Preparing EffectsProcessor");
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
