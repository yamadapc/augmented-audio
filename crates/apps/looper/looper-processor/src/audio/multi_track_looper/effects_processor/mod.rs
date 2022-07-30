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

use std::ops::Deref;

use basedrop::{Shared, SharedCell};

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
#[allow(clippy::enum_variant_names)]
pub enum EffectType {
    EffectTypeReverb = 0,
    EffectTypeDelay = 1,
    EffectTypeFilter = 2,
    EffectTypeBitCrusher = 3,
}

#[derive(Clone)]
pub struct EffectNodeState {
    node_index: NodeIndex,
    #[allow(unused)]
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

        let settings = *self.settings.get().deref();
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
        log::debug!("Rebuilding effects graph");
        let effects = self.effects.get();

        let mut last_index = self.graph_handle.input();
        for effect in &*effects {
            log::debug!(
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

    pub fn from_handle(handle: Shared<EffectsProcessorHandle>) -> Self {
        let graph = AudioProcessorGraph::from_handle(handle.graph_handle.clone());
        Self { graph, handle }
    }

    pub fn handle(&self) -> &Shared<EffectsProcessorHandle> {
        &self.handle
    }
}

impl AudioProcessor for EffectsProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        log::debug!("Preparing EffectsProcessor");
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
