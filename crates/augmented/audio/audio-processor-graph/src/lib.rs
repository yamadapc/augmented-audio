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

//! WIP - Draft of a version of https://github.com/RustAudio/dsp-chain which will work with the
//! `audio-processor-traits` crate (support for abstract `AudioBuffer` / `AudioProcessor`s).

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ops::Deref;

use daggy::Walker;
use thiserror::Error;

use audio_garbage_collector::{make_shared, make_shared_cell, Shared, SharedCell};
use audio_processor_traits::simple_processor::MonoAudioProcessor;
use audio_processor_traits::{
    AudioBuffer, AudioContext, AudioProcessor, AudioProcessorSettings, NoopAudioProcessor,
};
use augmented_oscillator::Oscillator;

#[cfg(test)]
mod test_allocator;

pub type NodeIndex = daggy::NodeIndex<u32>;
pub type ConnectionIndex = daggy::EdgeIndex<u32>;

/// Default static processor type
pub type DefaultProcessor = NoopAudioProcessor<f32>;

/// Non-generic AudioProcessorGraphImpl
pub type AudioProcessorGraph = AudioProcessorGraphImpl<DefaultProcessor>;

/// Non-generic AudioProcessorGraphHandleImpl
pub type AudioProcessorGraphHandle = AudioProcessorGraphHandleImpl<DefaultProcessor>;

struct BufferCell<BufferType>(UnsafeCell<BufferType>);

unsafe impl<BufferType> Sync for BufferCell<BufferType> {}

struct ProcessorCell<P>(UnsafeCell<P>);

unsafe impl<P> Sync for ProcessorCell<P> {}

pub struct AudioProcessorGraphHandleImpl<P> {
    input_node: NodeIndex,
    output_node: NodeIndex,
    dag: SharedCell<daggy::Dag<(), ()>>,
    process_order: SharedCell<Vec<NodeIndex>>,
    audio_processor_settings: SharedCell<Option<AudioProcessorSettings>>,
    processors: SharedCell<HashMap<NodeIndex, Shared<ProcessorCell<NodeType<P>>>>>,
    buffers: SharedCell<HashMap<ConnectionIndex, Shared<BufferCell<AudioBuffer<f32>>>>>,
}

impl<P: Send + 'static + AudioProcessor> AudioProcessorGraphHandleImpl<P> {
    pub fn add_node(&self, mut processor: NodeType<P>) -> NodeIndex {
        let mut processors = self.processors.get().deref().clone();
        let mut dag = self.dag.get().deref().clone();
        let index = dag.add_node(());

        if let Some(settings) = self.audio_processor_settings.get().deref() {
            let mut context = AudioContext::from(*settings);
            match processor {
                NodeType::Simple(ref mut processor) => processor.prepare(&mut context),
                NodeType::Static(ref mut processor) => processor.prepare(&mut context),
                _ => {}
            }
        }

        let processor_ref = make_shared(ProcessorCell(UnsafeCell::new(processor)));
        processors.insert(index, processor_ref);

        self.processors.set(make_shared(processors));
        self.dag.set(make_shared(dag));
        index
    }

    pub fn add_connection(
        &self,
        source: NodeIndex,
        destination: NodeIndex,
    ) -> Result<ConnectionIndex, AudioProcessorGraphError> {
        let mut buffers = self.buffers.get().deref().clone();

        let mut dag = self.dag.get().deref().clone();
        let edge = dag
            .add_edge(source, destination, ())
            .map_err(|_| AudioProcessorGraphError::WouldCycle)?;
        let new_order = daggy::petgraph::algo::toposort(&dag, None)
            .map_err(|_| AudioProcessorGraphError::WouldCycle)?;

        let mut buffer = AudioBuffer::empty();
        if let Some(settings) = self.audio_processor_settings.get().deref() {
            buffer.resize(settings.output_channels, settings.block_size);
        }

        let buffer = make_shared(BufferCell(UnsafeCell::new(buffer)));

        buffers.insert(edge, buffer);
        self.buffers.set(make_shared(buffers));

        self.dag.set(make_shared(dag));
        self.process_order.set(make_shared(new_order));

        Ok(edge)
    }

    pub fn clear(&self) {
        let mut dag = self.dag.get().deref().clone();
        dag.clear_edges();
        self.dag.set(make_shared(dag));
    }

    pub fn input(&self) -> NodeIndex {
        self.input_node
    }

    pub fn output(&self) -> NodeIndex {
        self.output_node
    }
}

#[derive(Debug, Error)]
pub enum AudioProcessorGraphError {
    #[error("Adding this connection would result in a cycle")]
    WouldCycle,
}

pub enum NodeType<P> {
    Simple(Box<dyn AudioProcessor<SampleType = f32> + Send>),
    Static(P),
    None,
}

impl From<Box<dyn AudioProcessor<SampleType = f32> + Send>> for NodeType<NoopAudioProcessor<f32>> {
    fn from(inner: Box<dyn AudioProcessor<SampleType = f32> + Send>) -> Self {
        NodeType::Simple(inner)
    }
}

pub struct AudioProcessorGraphImpl<P> {
    input_node: NodeIndex,
    output_node: NodeIndex,
    handle: Shared<AudioProcessorGraphHandleImpl<P>>,
    temporary_buffer: AudioBuffer<f32>,
}

impl<P: Send + 'static + AudioProcessor> Default for AudioProcessorGraphImpl<P> {
    fn default() -> Self {
        Self::new(daggy::Dag::default())
    }
}

impl<P: Send + 'static + AudioProcessor> AudioProcessorGraphImpl<P> {
    fn new(mut dag: daggy::Dag<(), ()>) -> Self {
        let _input_proc: NodeType<P> = NodeType::Simple(Box::<NoopAudioProcessor<f32>>::default());
        let input_node = dag.add_node(());
        let _output_proc: NodeType<P> = NodeType::Simple(Box::<NoopAudioProcessor<f32>>::default());
        let output_node = dag.add_node(());

        AudioProcessorGraphImpl {
            input_node,
            output_node,
            handle: make_shared(AudioProcessorGraphHandleImpl {
                input_node,
                output_node,
                dag: make_shared_cell(dag),
                process_order: make_shared_cell(Vec::new()),
                audio_processor_settings: make_shared_cell(None),
                processors: make_shared_cell(HashMap::new()),
                buffers: make_shared_cell(HashMap::new()),
            }),
            temporary_buffer: AudioBuffer::empty(),
        }
    }

    pub fn from_handle(handle: Shared<AudioProcessorGraphHandleImpl<P>>) -> Self {
        Self {
            input_node: handle.input_node,
            output_node: handle.output_node,
            handle,
            temporary_buffer: AudioBuffer::empty(),
        }
    }

    pub fn input(&self) -> NodeIndex {
        self.input_node
    }

    pub fn output(&self) -> NodeIndex {
        self.output_node
    }

    pub fn handle(&self) -> &Shared<AudioProcessorGraphHandleImpl<P>> {
        &self.handle
    }

    pub fn add_node(&mut self, processor: NodeType<P>) -> NodeIndex {
        self.handle.add_node(processor)
    }

    pub fn add_connection(
        &mut self,
        source: NodeIndex,
        destination: NodeIndex,
    ) -> Result<ConnectionIndex, AudioProcessorGraphError> {
        self.handle.add_connection(source, destination)
    }
}

impl<P: AudioProcessor<SampleType = f32>> AudioProcessor for AudioProcessorGraphImpl<P> {
    type SampleType = f32;

    fn prepare(&mut self, context: &mut AudioContext) {
        let settings = context.settings;
        self.temporary_buffer
            .resize(settings.output_channels(), settings.block_size());

        self.handle
            .audio_processor_settings
            .set(make_shared(Some(settings)));
        let buffers = self.handle.buffers.get();
        for buffer_ref in buffers.values() {
            let buffer = buffer_ref.deref().0.get();
            unsafe {
                (*buffer).resize(settings.output_channels(), settings.block_size());
            }
        }

        let handle = self.handle.deref();

        let processors = handle.processors.get();
        let process_order = handle.process_order.get();
        let process_order = process_order.deref();

        for node_index in process_order {
            if let Some(processor) = handle
                .dag
                .get()
                .node_weight(*node_index)
                .and(processors.get(node_index))
            {
                unsafe {
                    match &mut *processor.deref().0.get() {
                        NodeType::Simple(processor) => {
                            processor.prepare(context);
                        }
                        NodeType::Static(processor) => {
                            processor.prepare(context);
                        }
                        NodeType::None => {}
                    }
                }
            }
        }
    }

    fn process(&mut self, context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>) {
        let num_channels = data.num_channels();
        let num_samples = data.num_samples();
        // TODO: this is bad, but I'm not sure how to handle variable size buffers (maybe process multiple times)
        self.temporary_buffer.resize(num_channels, num_samples);

        let handle = self.handle.deref();
        let dag = handle.dag.get();
        let dag = dag.deref();
        let processors = handle.processors.get();
        let buffers = handle.buffers.get();
        let process_order = handle.process_order.get();
        let process_order = process_order.deref();

        // Push inputs in
        let mut outputs = dag.children(self.input_node);
        while let Some((connection_id, _)) = outputs.walk_next(dag) {
            if let Some(buffer_ref) = dag
                .edge_weight(connection_id)
                .and(buffers.get(&connection_id))
            {
                let buffer = unsafe { &mut *buffer_ref.deref().0.get() };
                buffer.resize(num_channels, num_samples);
                buffer.copy_from(data);
            }
        }

        for node_index in process_order {
            let node_index = *node_index;

            if node_index == self.input_node || node_index == self.output_node {
                continue;
            }

            // Silence the temporary buffer
            for sample in self.temporary_buffer.slice_mut() {
                *sample = 0.0
            }

            let inputs = dag.parents(node_index);
            for (connection_id, _) in inputs.iter(dag) {
                if let Some(buffer_ref) = dag
                    .edge_weight(connection_id)
                    .and(buffers.get(&connection_id))
                {
                    let buffer = buffer_ref.deref().0.get();
                    unsafe {
                        let buffer = &mut *buffer;
                        buffer.resize(data.num_channels(), data.num_samples());
                        self.temporary_buffer.add(buffer);
                    }
                }
            }

            if let Some(processor_ref) =
                dag.node_weight(node_index).and(processors.get(&node_index))
            {
                let processor = processor_ref.deref().0.get();
                let processor = unsafe { &mut *processor };

                match processor {
                    NodeType::Simple(processor) => {
                        processor.process(context, &mut self.temporary_buffer);
                    }
                    NodeType::Static(processor) => {
                        processor.process(context, &mut self.temporary_buffer);
                    }
                    NodeType::None => {}
                }
            }

            let mut outputs = dag.children(node_index);
            while let Some((connection_id, _)) = outputs.walk_next(dag) {
                if let Some(buffer_ref) = dag
                    .edge_weight(connection_id)
                    .and(buffers.get(&connection_id))
                {
                    let buffer = buffer_ref.deref().0.get();
                    let buffer = unsafe { &mut *buffer };
                    buffer.resize(num_channels, num_samples);
                    buffer.copy_from(&self.temporary_buffer);
                }
            }
        }

        // Push outputs out
        let inputs = dag.parents(self.output_node);

        // Clear input
        for d in data.slice_mut() {
            *d = 0.0;
        }

        for (connection_id, _) in inputs.iter(dag) {
            if let Some(buffer_ref) = dag
                .edge_weight(connection_id)
                .and(buffers.get(&connection_id))
            {
                let buffer = buffer_ref.deref().0.get();

                data.add(unsafe { &*buffer });
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use assert_no_alloc::assert_no_alloc;
    use audio_processor_testing_helpers::{assert_f_eq, test_level_equivalence};
    use audio_processor_testing_helpers::{rms_level, sine_buffer};

    use audio_processor_traits::audio_buffer::AudioBuffer;
    use audio_processor_traits::simple_processor::MonoCopyProcessor;
    use audio_processor_utility::gain::GainProcessor;
    use audio_processor_utility::pan::PanProcessor;
    use augmented_oscillator::Oscillator;

    use super::*;

    #[test]
    fn test_create_graph() {
        let _ = AudioProcessorGraph::default();
    }

    #[test]
    fn test_create_graph_and_add_node() {
        let mut graph = AudioProcessorGraph::default();
        graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(
            GainProcessor::default(),
        ))));
    }

    #[test]
    fn test_create_graph_and_add_2_nodes() {
        let mut graph = AudioProcessorGraph::default();
        let gain1 = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(
            GainProcessor::default(),
        ))));
        let gain2 = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(
            GainProcessor::default(),
        ))));
        let _connection_id = graph.add_connection(gain1, gain2).unwrap();
    }

    #[test]
    fn test_create_heterogeneous_graph() {
        let mut graph = AudioProcessorGraph::default();
        let gain = Box::new(MonoCopyProcessor::new(GainProcessor::default()));
        let gain = graph.add_node(NodeType::Simple(gain));
        let pan = graph.add_node(NodeType::Simple(Box::new(PanProcessor::default())));
        let _connection_id = graph.add_connection(gain, pan).unwrap();
    }

    #[test]
    fn test_process_empty_graph_is_silent() {
        type BufferType = AudioBuffer<f32>;

        let mut settings = AudioProcessorSettings::default();
        settings.input_channels = 1;
        settings.output_channels = 1;
        settings.block_size = 1000;
        let mut context = AudioContext::from(settings);

        let mut empty_buffer = BufferType::empty();
        empty_buffer.resize(1, 1000);
        let mut graph = AudioProcessorGraph::default();

        assert!((rms_level(empty_buffer.channel(0)) - 0.0).abs() < f32::EPSILON);

        let mut process_buffer = empty_buffer.clone();
        graph.prepare(&mut context);

        assert_no_alloc(|| {
            graph.process(&mut context, &mut process_buffer);
        });

        test_level_equivalence(
            empty_buffer.channel(0),
            process_buffer.channel(0),
            1000,
            1000,
            f32::EPSILON,
        );
    }

    #[test]
    fn test_empty_graph_passthrough() {
        let mut context = AudioContext::default();
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;
        let mut buffer = AudioBuffer::empty();
        buffer.resize(1, 4);
        buffer.set(0, 0, 1.0);
        buffer.set(0, 1, 2.0);
        buffer.set(0, 2, 3.0);
        buffer.set(0, 3, 4.0);

        let mut graph = AudioProcessorGraph::default();
        graph.add_connection(graph.input(), graph.output()).unwrap();
        graph.prepare(&mut context);
        assert_no_alloc(|| {
            graph.process(&mut context, &mut buffer);
        });
        assert_f_eq!(*buffer.get(0, 0), 1.0);
        assert_f_eq!(*buffer.get(0, 1), 2.0);
        assert_f_eq!(*buffer.get(0, 2), 3.0);
        assert_f_eq!(*buffer.get(0, 3), 4.0);
    }

    #[test]
    fn test_single_node_graph() {
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;
        context.settings.block_size = 4;

        let mut buffer = AudioBuffer::empty();
        buffer.resize(1, 4);
        buffer.set(0, 0, 1.0);
        buffer.set(0, 1, 2.0);
        buffer.set(0, 2, 3.0);
        buffer.set(0, 3, 4.0);

        struct Sum10Node {}
        impl MonoAudioProcessor for Sum10Node {
            type SampleType = f32;
            fn m_process(
                &mut self,
                _context: &mut AudioContext,
                sample: Self::SampleType,
            ) -> Self::SampleType {
                sample + 10.0
            }
        }

        let sum_10_node = MonoCopyProcessor::new(Sum10Node {});

        let mut graph = AudioProcessorGraph::default();
        let sum_10_node = graph.add_node(NodeType::Simple(Box::new(sum_10_node)));
        graph.add_connection(graph.input(), sum_10_node).unwrap();
        graph.add_connection(sum_10_node, graph.output()).unwrap();
        graph.prepare(&mut context);
        assert_no_alloc(|| {
            graph.process(&mut context, &mut buffer);
        });
        assert_f_eq!(*buffer.get(0, 0), 11.0);
        assert_f_eq!(*buffer.get(0, 1), 12.0);
        assert_f_eq!(*buffer.get(0, 2), 13.0);
        assert_f_eq!(*buffer.get(0, 3), 14.0);
    }

    #[test]
    fn test_series_node_graph() {
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;
        context.settings.block_size = 4;

        let mut buffer = AudioBuffer::empty();
        buffer.resize(1, 4);
        buffer.set(0, 0, 1.0);
        buffer.set(0, 1, 2.0);
        buffer.set(0, 2, 3.0);
        buffer.set(0, 3, 4.0);

        #[derive(Clone)]
        struct Mult10Node {}
        impl MonoAudioProcessor for Mult10Node {
            type SampleType = f32;
            fn m_process(
                &mut self,
                _context: &mut AudioContext,
                sample: Self::SampleType,
            ) -> Self::SampleType {
                sample * 10.0
            }
        }

        let mult_10_node = Mult10Node {};

        let mut graph = AudioProcessorGraph::default();
        let node1 = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(
            mult_10_node.clone(),
        ))));
        let node2 = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(
            mult_10_node,
        ))));
        graph.add_connection(graph.input(), node1).unwrap();
        graph.add_connection(node1, node2).unwrap();
        graph.add_connection(node2, graph.output()).unwrap();
        graph.prepare(&mut context);
        assert_no_alloc(|| {
            graph.process(&mut context, &mut buffer);
        });
        assert_f_eq!(*buffer.get(0, 0), 100.0);
        assert_f_eq!(*buffer.get(0, 1), 200.0);
        assert_f_eq!(*buffer.get(0, 2), 300.0);
        assert_f_eq!(*buffer.get(0, 3), 400.0);
    }

    #[test]
    fn test_buffer_in_series() {
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;
        let mut buffer = AudioBuffer::empty();
        buffer.resize(1, 4);
        buffer.set(0, 0, 1.0);
        buffer.set(0, 1, 2.0);
        buffer.set(0, 2, 3.0);
        buffer.set(0, 3, 4.0);

        #[derive(Clone)]
        struct Mult10Node {}
        impl AudioProcessor for Mult10Node {
            type SampleType = f32;
            fn process(&mut self, _context: &mut AudioContext, buffer: &mut AudioBuffer<f32>) {
                for sample in buffer.slice_mut() {
                    *sample *= 10.0
                }
            }
        }

        let mult_10_node = Mult10Node {};

        let mut graph = AudioProcessorGraph::default();
        let node1 = graph.add_node(NodeType::Simple(Box::new(mult_10_node.clone())));
        let node2 = graph.add_node(NodeType::Simple(Box::new(mult_10_node)));
        graph.add_connection(graph.input(), node1).unwrap();
        graph.add_connection(node1, node2).unwrap();
        graph.add_connection(node2, graph.output()).unwrap();
        graph.prepare(&mut context);
        assert_no_alloc(|| {
            graph.process(&mut context, &mut buffer);
        });
        assert_f_eq!(*buffer.get(0, 0), 100.0);
        assert_f_eq!(*buffer.get(0, 1), 200.0);
        assert_f_eq!(*buffer.get(0, 2), 300.0);
        assert_f_eq!(*buffer.get(0, 3), 400.0);
    }

    #[test]
    fn test_buffer_in_parallel() {
        let mut settings = AudioProcessorSettings::default();
        settings.input_channels = 1;
        settings.output_channels = 1;
        settings.block_size = 4;

        let mut context = AudioContext::from(settings);
        let mut buffer = AudioBuffer::empty();
        buffer.resize(1, 4);
        buffer.set(0, 0, 1.0);
        buffer.set(0, 1, 2.0);
        buffer.set(0, 2, 3.0);
        buffer.set(0, 3, 4.0);

        #[derive(Clone)]
        struct Mult10Node {}
        impl AudioProcessor for Mult10Node {
            type SampleType = f32;
            fn process(&mut self, _context: &mut AudioContext, buffer: &mut AudioBuffer<f32>) {
                for sample in buffer.slice_mut() {
                    *sample *= 10.0
                }
            }
        }

        let mult_10_node = Mult10Node {};

        let mut graph = AudioProcessorGraph::default();
        let node1 = graph.add_node(NodeType::Simple(Box::new(mult_10_node.clone())));
        let node2 = graph.add_node(NodeType::Simple(Box::new(mult_10_node)));
        graph.add_connection(graph.input(), node1).unwrap();
        graph.add_connection(node1, graph.output()).unwrap();
        graph.add_connection(graph.input(), node2).unwrap();
        graph.add_connection(node2, graph.output()).unwrap();
        graph.prepare(&mut context);

        assert_no_alloc(|| {
            graph.process(&mut context, &mut buffer);
        });

        assert_f_eq!(*buffer.get(0, 0), 20.0);
        assert_f_eq!(*buffer.get(0, 1), 40.0);
        assert_f_eq!(*buffer.get(0, 2), 60.0);
        assert_f_eq!(*buffer.get(0, 3), 80.0);
    }

    #[test]
    fn test_more_complex_graph() {
        // input -> mult-10 -> mult-10 -> output
        //     \> mult-10 -> mult-10 ----/
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;
        context.settings.block_size = 4;

        let mut buffer = AudioBuffer::empty();
        buffer.resize(1, 4);
        buffer.set(0, 0, 1.0);
        buffer.set(0, 1, 2.0);
        buffer.set(0, 2, 3.0);
        buffer.set(0, 3, 4.0);

        #[derive(Clone)]
        struct Mult10Node {}
        impl AudioProcessor for Mult10Node {
            type SampleType = f32;
            fn process(&mut self, _context: &mut AudioContext, buffer: &mut AudioBuffer<f32>) {
                for sample in buffer.slice_mut() {
                    *sample *= 10.0
                }
            }
        }

        let mult_10_node = Mult10Node {};

        let mut graph = AudioProcessorGraph::default();
        let node_a1 = graph.add_node(NodeType::Simple(Box::new(mult_10_node.clone())));
        let node_a2 = graph.add_node(NodeType::Simple(Box::new(mult_10_node.clone())));
        let node_b1 = graph.add_node(NodeType::Simple(Box::new(mult_10_node.clone())));
        let node_b2 = graph.add_node(NodeType::Simple(Box::new(mult_10_node)));
        graph.add_connection(graph.input(), node_a1).unwrap();
        graph.add_connection(node_a1, node_a2).unwrap();
        graph.add_connection(node_a2, graph.output()).unwrap();
        graph.add_connection(graph.input(), node_b1).unwrap();
        graph.add_connection(node_b1, node_b2).unwrap();
        graph.add_connection(node_b2, graph.output()).unwrap();
        graph.prepare(&mut context);
        assert_no_alloc(|| {
            graph.process(&mut context, &mut buffer);
        });
        assert_f_eq!(*buffer.get(0, 0), 200.0);
        assert_f_eq!(*buffer.get(0, 1), 400.0);
        assert_f_eq!(*buffer.get(0, 2), 600.0);
        assert_f_eq!(*buffer.get(0, 3), 800.0);
    }

    #[test]
    fn test_30_node_graph() {
        // input -> mult-10 -> mult-10 -> output
        //     \> mult-10 -> mult-10 ----/
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;
        context.settings.block_size = 4;

        let mut buffer = AudioBuffer::empty();
        buffer.resize(1, 4);
        buffer.set(0, 0, 1.0);
        buffer.set(0, 1, 2.0);
        buffer.set(0, 2, 3.0);
        buffer.set(0, 3, 4.0);

        #[derive(Clone)]
        struct Mult10Node {}
        impl AudioProcessor for Mult10Node {
            type SampleType = f32;
            fn process(&mut self, _context: &mut AudioContext, buffer: &mut AudioBuffer<f32>) {
                for sample in buffer.slice_mut() {
                    *sample *= 10.0
                }
            }
        }

        let mult_10_node = Mult10Node {};

        let mut graph = AudioProcessorGraph::default();
        for _i in 0..10 {
            let mut current_idx = graph.input();
            for _i in 0..3 {
                let node = graph.add_node(NodeType::Simple(Box::new(mult_10_node.clone())));
                graph.add_connection(current_idx, node).unwrap();
                current_idx = node;
            }
            graph.add_connection(current_idx, graph.output()).unwrap();
        }
        graph.prepare(&mut context);
        assert_no_alloc(|| {
            graph.process(&mut context, &mut buffer);
        });
        assert_f_eq!(*buffer.get(0, 0), 10000.0);
        assert_f_eq!(*buffer.get(0, 1), 20000.0);
        assert_f_eq!(*buffer.get(0, 2), 30000.0);
        assert_f_eq!(*buffer.get(0, 3), 40000.0);
    }

    #[test]
    fn test_process_empty_graph_passes_through_sine() {
        type BufferType = AudioBuffer<f32>;

        let mut settings = AudioProcessorSettings::default();
        settings.set_input_channels(1);
        settings.set_output_channels(1);
        let sine_buffer: BufferType = BufferType::from_interleaved(
            1,
            &sine_buffer(settings.sample_rate, 440.0, Duration::from_millis(3)),
        );
        let mut context = AudioContext::from(settings);
        assert_eq!(sine_buffer.num_channels(), 1);

        let mut graph = AudioProcessorGraph::default();
        graph.add_connection(graph.input(), graph.output()).unwrap();

        let mut process_buffer = sine_buffer.clone();
        graph.prepare(&mut context);
        assert_no_alloc(|| {
            graph.process(&mut context, &mut process_buffer);
        });

        test_level_equivalence(
            sine_buffer.channel(0),
            process_buffer.channel(0),
            settings.block_size(),
            settings.block_size(),
            f32::EPSILON,
        );
    }

    #[test]
    fn test_process_sine_generator_is_silent_if_disconnected() {
        type BufferType = AudioBuffer<f32>;

        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;
        context.settings.block_size = 1000;

        let mut oscillator = Oscillator::sine(settings.sample_rate);
        oscillator.set_frequency(440.0);
        let oscillator = OscillatorProcessor { oscillator };

        let mut empty_buffer: BufferType = AudioBuffer::empty();
        empty_buffer.resize(1, 1000);

        let mut process_buffer = AudioBuffer::empty();
        process_buffer.resize(1, 1000);

        // Unconnected
        let mut graph = AudioProcessorGraph::default();
        graph.prepare(&mut context);
        let _oscillator_idx = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(
            oscillator,
        ))));
        assert_no_alloc(|| {
            graph.process(&mut context, &mut process_buffer);
        });

        test_level_equivalence(
            empty_buffer.channel(0),
            process_buffer.channel(0),
            1000,
            1000,
            f32::EPSILON,
        );
    }

    #[test]
    fn test_process_sine_generator_is_silent_if_only_connected_to_input() {
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;
        context.settings.block_size = 1000;

        let mut oscillator = Oscillator::sine(settings.sample_rate);
        oscillator.set_frequency(440.0);
        let oscillator = OscillatorProcessor { oscillator };

        let mut empty_buffer = AudioBuffer::empty();
        empty_buffer.resize(1, 1000);

        let mut process_buffer = AudioBuffer::empty();
        process_buffer.resize(1, 1000);

        // Connected to input only
        let mut graph = AudioProcessorGraph::default();
        graph.prepare(&mut context);
        let oscillator_idx = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(
            oscillator,
        ))));
        graph
            .add_connection(graph.input_node, oscillator_idx)
            .unwrap();
        assert_no_alloc(|| {
            graph.process(&mut context, &mut process_buffer);
        });

        test_level_equivalence(
            empty_buffer.channel(0),
            process_buffer.channel(0),
            1000,
            1000,
            f32::EPSILON,
        );
    }

    #[test]
    fn test_two_nodes_get_summed_if_connected_to_output() {
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;

        struct Node1;
        impl MonoAudioProcessor for Node1 {
            type SampleType = f32;
            fn m_process(
                &mut self,
                _context: &mut AudioContext,
                sample: Self::SampleType,
            ) -> Self::SampleType {
                sample + 1.0
            }
        }

        struct Node2;
        impl MonoAudioProcessor for Node2 {
            type SampleType = f32;
            fn m_process(
                &mut self,
                _context: &mut AudioContext,
                sample: Self::SampleType,
            ) -> Self::SampleType {
                sample + 2.0
            }
        }

        let node1 = Node1 {};
        let node2 = Node2 {};

        let mut graph = AudioProcessorGraph::default();
        graph.prepare(&mut context);

        let input_idx = graph.input();
        let output_idx = graph.output();
        let node1_idx = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(node1))));
        let node2_idx = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(node2))));
        graph.add_connection(input_idx, node1_idx).unwrap();
        graph.add_connection(input_idx, node2_idx).unwrap();
        graph.add_connection(node1_idx, output_idx).unwrap();
        graph.add_connection(node2_idx, output_idx).unwrap();

        let mut process_buffer = AudioBuffer::empty();
        process_buffer.resize(1, 3);
        assert_no_alloc(|| {
            graph.process(&mut context, &mut process_buffer);
        });
        let output = process_buffer.channel(0).to_vec();
        assert_eq!(output, vec![3.0, 3.0, 3.0]);
    }

    #[test]
    fn test_process_sine_generator_in_the_graph_produces_sine() {
        type BufferType = AudioBuffer<f32>;
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        context.settings.input_channels = 1;
        context.settings.output_channels = 1;
        context.settings.block_size = 1000;

        let mut oscillator = Oscillator::sine(settings.sample_rate);
        oscillator.set_frequency(440.0);
        let oscillator = OscillatorProcessor { oscillator };

        let reference_sine: BufferType = BufferType::from_interleaved(
            1,
            &sine_buffer(
                settings.sample_rate,
                440.0,
                Duration::from_secs_f32((1.0 / settings.sample_rate) * 1000.0),
            ),
        );

        let mut process_buffer = AudioBuffer::empty();
        process_buffer.resize(1, 1000);

        let mut graph = AudioProcessorGraph::default();
        graph.prepare(&mut context);
        let oscillator_idx = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(
            oscillator,
        ))));
        graph
            .add_connection(graph.input_node, oscillator_idx)
            .unwrap();
        graph
            .add_connection(oscillator_idx, graph.output_node)
            .unwrap();
        assert_no_alloc(|| {
            graph.process(&mut context, &mut process_buffer);
        });

        test_level_equivalence(
            reference_sine.channel(0),
            process_buffer.channel(0),
            1000,
            1000,
            f32::EPSILON,
        );
    }
}

// Testing helper
#[derive(Clone)]
pub struct OscillatorProcessor {
    pub oscillator: Oscillator<f32>,
}

impl MonoAudioProcessor for OscillatorProcessor {
    type SampleType = f32;

    fn m_prepare(&mut self, context: &mut AudioContext) {
        self.oscillator
            .set_sample_rate(context.settings.sample_rate);
    }

    fn m_process(
        &mut self,
        _context: &mut AudioContext,
        _sample: Self::SampleType,
    ) -> Self::SampleType {
        self.oscillator.next_sample()
    }
}
