use std::cell::UnsafeCell;
use std::ops::Deref;

use concread::hashmap::HashMap;
use daggy::Walker;
use num_traits::Zero;
use thiserror::Error;

use audio_garbage_collector::{make_shared, make_shared_cell, Shared, SharedCell};
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, Float, InterleavedAudioBuffer,
    NoopAudioProcessor, ObjectAudioProcessor,
};
use augmented_oscillator::Oscillator;

pub type NodeIndex = daggy::NodeIndex<u32>;
pub type ConnectionIndex = daggy::EdgeIndex<u32>;

struct BufferCell<BufferType>(UnsafeCell<BufferType>);

unsafe impl<BufferType> Sync for BufferCell<BufferType> {}

struct ProcessorCell<BufferType: OwnedAudioBuffer>(UnsafeCell<NodeType<BufferType>>);

unsafe impl<BufferType: OwnedAudioBuffer> Sync for ProcessorCell<BufferType> {}

fn make_processor_cell<BufferType: OwnedAudioBuffer + 'static>(
    processor: NodeType<BufferType>,
) -> Shared<ProcessorCell<BufferType>> {
    make_shared(ProcessorCell(UnsafeCell::new(processor)))
}

pub struct AudioProcessorGraphHandle<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: 'static,
{
    dag: SharedCell<daggy::Dag<(), ()>>,
    process_order: SharedCell<Vec<NodeIndex>>,
    audio_processor_settings: SharedCell<Option<AudioProcessorSettings>>,
    processors: HashMap<NodeIndex, Shared<ProcessorCell<BufferType>>>,
    buffers: HashMap<ConnectionIndex, Shared<BufferCell<BufferType>>>,
}

impl<BufferType> AudioProcessorGraphHandle<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: 'static + Copy + Float,
{
    pub fn add_node(&self, mut processor: NodeType<BufferType>) -> NodeIndex {
        let mut tx = self.processors.write();
        let mut dag = self.dag.get().deref().clone();
        let index = dag.add_node(());

        if let Some(settings) = self.audio_processor_settings.get().deref() {
            match processor {
                NodeType::Simple(ref mut processor) => processor.s_prepare(*settings),
                NodeType::Buffer(ref mut processor) => processor.prepare_obj(*settings),
            }
        }

        let processor_ref = make_processor_cell(processor);
        tx.insert(index, processor_ref);
        tx.commit();

        self.dag.set(make_shared(dag));
        index
    }

    pub fn add_connection(
        &self,
        source: NodeIndex,
        destination: NodeIndex,
    ) -> Result<ConnectionIndex, AudioProcessorGraphError> {
        let mut tx = self.buffers.write();

        let mut dag = self.dag.get().deref().clone();
        let edge = dag
            .add_edge(source, destination, ())
            .map_err(|_| AudioProcessorGraphError::WouldCycle)?;
        let new_order = daggy::petgraph::algo::toposort(&dag, None)
            .map_err(|_| AudioProcessorGraphError::WouldCycle)?;

        let mut buffer = BufferType::new();
        if let Some(settings) = self.audio_processor_settings.get().deref() {
            buffer.resize(
                settings.input_channels,
                settings.block_size as usize,
                BufferType::SampleType::zero(),
            );
        }

        let buffer = make_shared(BufferCell(UnsafeCell::new(buffer)));

        tx.insert(edge, buffer);
        tx.commit();

        self.dag.set(make_shared(dag));
        self.process_order.set(make_shared(new_order));

        Ok(edge)
    }
}

#[derive(Debug, Error)]
pub enum AudioProcessorGraphError {
    #[error("Adding this connection would result in a cycle")]
    WouldCycle,
}

pub enum NodeType<BufferType: OwnedAudioBuffer> {
    Simple(Box<dyn SimpleAudioProcessor<SampleType = BufferType::SampleType> + Send>),
    Buffer(Box<dyn ObjectAudioProcessor<BufferType> + Send>),
}

impl<BufferType: OwnedAudioBuffer>
    From<Box<dyn SimpleAudioProcessor<SampleType = BufferType::SampleType> + Send>>
    for NodeType<BufferType>
{
    fn from(
        inner: Box<dyn SimpleAudioProcessor<SampleType = BufferType::SampleType> + Send>,
    ) -> Self {
        NodeType::Simple(inner)
    }
}

impl<BufferType: OwnedAudioBuffer> From<Box<dyn ObjectAudioProcessor<BufferType> + Send>>
    for NodeType<BufferType>
{
    fn from(inner: Box<dyn ObjectAudioProcessor<BufferType> + Send>) -> Self {
        NodeType::Buffer(inner)
    }
}

// pub type NodeType<SampleType> = Box<dyn SimpleAudioProcessor<SampleType = SampleType> + Send>;

pub struct AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: 'static,
{
    input_node: NodeIndex,
    output_node: NodeIndex,
    handle: Shared<AudioProcessorGraphHandle<BufferType>>,
    temporary_buffer: BufferType,
}

impl<BufferType> Default for AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: Copy + Send + 'static + Float,
{
    fn default() -> Self {
        Self::new(daggy::Dag::default())
    }
}

impl<BufferType> AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: Copy + Send + 'static + Float,
{
    fn new(mut dag: daggy::Dag<(), ()>) -> Self {
        let input_proc: NodeType<BufferType> =
            NodeType::Simple(Box::new(NoopAudioProcessor::default()));
        let input_node = dag.add_node(());
        let output_proc: NodeType<BufferType> =
            NodeType::Simple(Box::new(NoopAudioProcessor::default()));
        let output_node = dag.add_node(());

        let processors = HashMap::new();
        let mut tx = processors.write();
        tx.insert(input_node, make_processor_cell(input_proc));
        tx.insert(output_node, make_processor_cell(output_proc));
        tx.commit();

        AudioProcessorGraph {
            input_node,
            output_node,
            handle: make_shared(AudioProcessorGraphHandle {
                dag: make_shared_cell(dag),
                process_order: make_shared_cell(Vec::new()),
                audio_processor_settings: make_shared_cell(None),
                processors: HashMap::new(),
                buffers: HashMap::new(),
            }),
            temporary_buffer: BufferType::new(),
        }
    }

    pub fn input(&self) -> NodeIndex {
        self.input_node
    }

    pub fn output(&self) -> NodeIndex {
        self.output_node
    }

    pub fn handle(&self) -> &Shared<AudioProcessorGraphHandle<BufferType>> {
        &self.handle
    }

    pub fn add_node(&mut self, processor: NodeType<BufferType>) -> NodeIndex {
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

impl<BufferType> AudioProcessor for AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer + Send,
    BufferType::SampleType: Copy + Float,
{
    type SampleType = BufferType::SampleType;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.temporary_buffer.resize(
            settings.input_channels(),
            settings.block_size(),
            BufferType::SampleType::zero(),
        );

        self.handle
            .audio_processor_settings
            .set(make_shared(Some(settings)));
        let buffers_tx = self.handle.buffers.read();
        for buffer_ref in buffers_tx.values() {
            let buffer = buffer_ref.deref().0.get();
            unsafe {
                (*buffer).resize(
                    settings.input_channels(),
                    settings.block_size(),
                    BufferType::SampleType::zero(),
                );
            }
        }

        let handle = self.handle.deref();

        let tx = handle.processors.read();
        let process_order = handle.process_order.get();
        let process_order = process_order.deref();

        for node_index in process_order {
            if let Some(processor) = handle
                .dag
                .get()
                .node_weight(*node_index)
                .and(tx.get(node_index))
            {
                unsafe {
                    match &mut *processor.deref().0.get() {
                        NodeType::Simple(processor) => {
                            processor.s_prepare(settings);
                        }
                        NodeType::Buffer(processor) => {
                            processor.prepare_obj(settings);
                        }
                    }
                }
            }
        }
    }

    fn process<InputBufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut InputBufferType,
    ) {
        let handle = self.handle.deref();
        let dag = handle.dag.get();
        let dag = dag.deref();
        let processors = handle.processors.read();
        let buffers = handle.buffers.read();
        let process_order = handle.process_order.get();
        let process_order = process_order.deref();

        // Push inputs in
        let mut outputs = dag.children(self.input_node);
        while let Some((connection_id, _)) = outputs.walk_next(dag) {
            if let Some(buffer_ref) = dag
                .edge_weight(connection_id)
                .and(buffers.get(&connection_id))
            {
                let buffer = buffer_ref.deref().0.get();
                unsafe {
                    copy_buffer(data, &mut *buffer);
                }
            }
        }

        for node_index in process_order {
            let node_index = *node_index;

            if node_index == self.input_node || node_index == self.output_node {
                continue;
            }

            // Silence the temporary buffer
            for sample in self.temporary_buffer.slice_mut() {
                *sample = BufferType::SampleType::zero();
            }

            let inputs = dag.parents(node_index);
            for (connection_id, _) in inputs.iter(dag) {
                if let Some(buffer_ref) = dag
                    .edge_weight(connection_id)
                    .and(buffers.get(&connection_id))
                {
                    let buffer = buffer_ref.deref().0.get();

                    unsafe {
                        for (s, d) in (&*buffer)
                            .slice()
                            .iter()
                            .zip(self.temporary_buffer.slice_mut())
                            .take(data.num_samples() * data.num_channels())
                        {
                            *d = *d + *s;
                        }
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
                        for frame in self.temporary_buffer.frames_mut().take(data.num_samples()) {
                            processor.s_process_frame(frame);
                        }
                    }
                    NodeType::Buffer(processor) => {
                        // TODO: This is wrong as data might be smaller than temporary buffer
                        processor.process_obj(&mut self.temporary_buffer);
                    }
                }
            }

            let mut outputs = dag.children(node_index);
            while let Some((connection_id, _)) = outputs.walk_next(dag) {
                if let Some(buffer_ref) = dag
                    .edge_weight(connection_id)
                    .and(buffers.get(&connection_id))
                {
                    let buffer = buffer_ref.deref().0.get();
                    unsafe {
                        copy_buffer(&self.temporary_buffer, &mut *buffer);
                    }
                }
            }
        }

        // Push outputs out
        let inputs = dag.parents(self.output_node);

        // Clear input
        for d in data.slice_mut() {
            *d = BufferType::SampleType::zero();
        }

        for (connection_id, _) in inputs.iter(dag) {
            if let Some(buffer_ref) = dag
                .edge_weight(connection_id)
                .and(buffers.get(&connection_id))
            {
                let buffer = buffer_ref.deref().0.get();

                unsafe {
                    for (s, d) in (&*buffer).slice().iter().zip(data.slice_mut()) {
                        *d = *s + *d;
                    }
                }
            }
        }
    }
}

fn copy_buffer<SampleType, InputBufferType, OutputBufferType>(
    source: &InputBufferType,
    destination: &mut OutputBufferType,
) where
    SampleType: Copy,
    InputBufferType: AudioBuffer<SampleType = SampleType>,
    OutputBufferType: AudioBuffer<SampleType = SampleType>,
{
    let src = source.slice();
    let dest = destination.slice_mut();
    for (s, d) in src.iter().zip(dest) {
        *d = *s;
    }
}

#[cfg(test)]
mod test {
    use std::iter::Sum;
    use std::time::Duration;

    use audio_processor_testing_helpers::{assert_f_eq, test_level_equivalence};
    use audio_processor_testing_helpers::{rms_level, sine_buffer};

    use audio_processor_traits::audio_buffer::VecAudioBuffer;
    use audio_processor_utility::gain::GainProcessor;
    use audio_processor_utility::pan::PanProcessor;
    use augmented_oscillator::Oscillator;

    use super::*;

    #[test]
    fn test_create_graph() {
        let _ = AudioProcessorGraph::<VecAudioBuffer<f32>>::default();
    }

    #[test]
    fn test_create_graph_and_add_node() {
        let mut graph = AudioProcessorGraph::<VecAudioBuffer<f32>>::default();
        graph.add_node(NodeType::Simple(Box::new(GainProcessor::default())));
    }

    #[test]
    fn test_create_graph_and_add_2_nodes() {
        let mut graph = AudioProcessorGraph::<VecAudioBuffer<f32>>::default();
        let gain1 = graph.add_node(NodeType::Simple(Box::new(GainProcessor::default())));
        let gain2 = graph.add_node(NodeType::Simple(Box::new(GainProcessor::default())));
        let _connection_id = graph.add_connection(gain1, gain2).unwrap();
    }

    #[test]
    fn test_create_heterogeneous_graph() {
        type BufferType = VecAudioBuffer<f32>;

        let mut graph = AudioProcessorGraph::<BufferType>::default();
        let gain = Box::new(GainProcessor::default());
        let gain = graph.add_node(NodeType::Simple(gain));
        let pan = graph.add_node(NodeType::Simple(Box::new(PanProcessor::default())));
        let _connection_id = graph.add_connection(gain, pan).unwrap();
    }

    #[test]
    fn test_process_empty_graph_is_silent() {
        type BufferType = VecAudioBuffer<f32>;

        let settings = AudioProcessorSettings::default();

        let mut empty_buffer = BufferType::new();
        empty_buffer.resize(1, 1000, 0.0);
        let mut graph = AudioProcessorGraph::<BufferType>::default();

        assert!(rms_level(empty_buffer.slice()) - 0.0 < f32::EPSILON);

        let mut process_buffer = empty_buffer.clone();
        graph.prepare(settings);
        graph.process(&mut process_buffer);

        test_level_equivalence(
            empty_buffer.slice(),
            process_buffer.slice(),
            1000,
            1000,
            f32::EPSILON,
        );
    }

    #[test]
    fn test_empty_graph_passthrough() {
        type BufferType = VecAudioBuffer<f32>;
        let settings = AudioProcessorSettings::default();
        let mut buffer = VecAudioBuffer::new();
        buffer.resize(1, 4, 0.0);
        buffer.set(0, 0, 1.0);
        buffer.set(0, 1, 2.0);
        buffer.set(0, 2, 3.0);
        buffer.set(0, 3, 4.0);

        let mut graph = AudioProcessorGraph::<BufferType>::default();
        graph.add_connection(graph.input(), graph.output());
        graph.prepare(settings);
        graph.process(&mut buffer);
        println!("{:?}", buffer);
        assert_f_eq!(*buffer.get(0, 0), 1.0);
        assert_f_eq!(*buffer.get(0, 1), 2.0);
        assert_f_eq!(*buffer.get(0, 2), 3.0);
        assert_f_eq!(*buffer.get(0, 3), 4.0);
    }

    #[test]
    fn test_single_node_graph() {
        type BufferType = VecAudioBuffer<f32>;
        let settings = AudioProcessorSettings::default();
        let mut buffer = VecAudioBuffer::new();
        buffer.resize(1, 4, 0.0);
        buffer.set(0, 0, 1.0);
        buffer.set(0, 1, 2.0);
        buffer.set(0, 2, 3.0);
        buffer.set(0, 3, 4.0);

        struct Sum10Node {}
        impl SimpleAudioProcessor for Sum10Node {
            type SampleType = f32;
            fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
                sample + 10.0
            }
        }

        let sum_10_node = Sum10Node {};

        let mut graph = AudioProcessorGraph::<BufferType>::default();
        let sum_10_node = graph.add_node(NodeType::Simple(Box::new(sum_10_node)));
        graph.add_connection(graph.input(), sum_10_node);
        graph.add_connection(sum_10_node, graph.output());
        graph.prepare(settings);
        graph.process(&mut buffer);
        println!("{:?}", buffer);
        assert_f_eq!(*buffer.get(0, 0), 11.0);
        assert_f_eq!(*buffer.get(0, 1), 12.0);
        assert_f_eq!(*buffer.get(0, 2), 13.0);
        assert_f_eq!(*buffer.get(0, 3), 14.0);
    }

    #[test]
    fn test_process_empty_graph_passes_through_sine() {
        type BufferType = VecAudioBuffer<f32>;

        let mut settings = AudioProcessorSettings::default();
        settings.set_input_channels(1);
        settings.set_output_channels(1);
        let sine_buffer: BufferType =
            sine_buffer(settings.sample_rate, 440.0, Duration::from_millis(3)).into();
        assert_eq!(sine_buffer.num_channels(), 1);

        let mut graph = AudioProcessorGraph::<BufferType>::default();
        graph.add_connection(graph.input(), graph.output());

        let mut process_buffer = sine_buffer.clone();
        graph.prepare(settings);
        graph.process(&mut process_buffer);

        test_level_equivalence(
            sine_buffer.slice(),
            process_buffer.slice(),
            settings.block_size(),
            settings.block_size(),
            f32::EPSILON,
        );
    }

    #[test]
    fn test_process_sine_generator_is_silent_if_disconnected() {
        type BufferType = VecAudioBuffer<f32>;
        let settings = AudioProcessorSettings::default();

        let mut oscillator = Oscillator::sine(settings.sample_rate);
        oscillator.set_frequency(440.0);
        let oscillator = OscillatorProcessor { oscillator };

        let mut empty_buffer: BufferType = VecAudioBuffer::new();
        empty_buffer.resize(1, 1000, 0.0);

        let mut process_buffer = VecAudioBuffer::new();
        process_buffer.resize(1, 1000, 0.0);

        // Unconnected
        let mut graph = AudioProcessorGraph::<BufferType>::default();
        graph.prepare(settings);
        let _oscillator_idx = graph.add_node(NodeType::Simple(Box::new(oscillator)));
        graph.process(&mut process_buffer);

        test_level_equivalence(
            empty_buffer.slice(),
            process_buffer.slice(),
            1000,
            1000,
            f32::EPSILON,
        );
    }

    #[test]
    fn test_process_sine_generator_is_silent_if_only_connected_to_input() {
        type BufferType = VecAudioBuffer<f32>;
        let settings = AudioProcessorSettings::default();

        let mut oscillator = Oscillator::sine(settings.sample_rate);
        oscillator.set_frequency(440.0);
        let oscillator = OscillatorProcessor { oscillator };

        let mut empty_buffer: BufferType = VecAudioBuffer::new();
        empty_buffer.resize(1, 1000, 0.0);

        let mut process_buffer = VecAudioBuffer::new();
        process_buffer.resize(1, 1000, 0.0);

        // Connected to input only
        let mut graph = AudioProcessorGraph::<BufferType>::default();
        graph.prepare(settings);
        let oscillator_idx = graph.add_node(NodeType::Simple(Box::new(oscillator)));
        graph
            .add_connection(graph.input_node, oscillator_idx)
            .unwrap();
        graph.process(&mut process_buffer);

        test_level_equivalence(
            empty_buffer.slice(),
            process_buffer.slice(),
            1000,
            1000,
            f32::EPSILON,
        );
    }

    #[test]
    fn test_two_nodes_get_summed_if_connected_to_output() {
        type BufferType = VecAudioBuffer<f32>;
        let settings = AudioProcessorSettings::default();

        struct Node1;
        impl SimpleAudioProcessor for Node1 {
            type SampleType = f32;
            fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
                sample + 1.0
            }
        }

        struct Node2;
        impl SimpleAudioProcessor for Node2 {
            type SampleType = f32;
            fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
                sample + 2.0
            }
        }

        let node1 = Node1 {};
        let node2 = Node2 {};

        let mut graph = AudioProcessorGraph::<BufferType>::default();
        graph.prepare(settings);

        let input_idx = graph.input();
        let output_idx = graph.output();
        let node1_idx = graph.add_node(NodeType::Simple(Box::new(node1)));
        let node2_idx = graph.add_node(NodeType::Simple(Box::new(node2)));
        graph.add_connection(input_idx, node1_idx);
        graph.add_connection(input_idx, node2_idx);
        graph.add_connection(node1_idx, output_idx);
        graph.add_connection(node2_idx, output_idx);

        let mut process_buffer = VecAudioBuffer::new();
        process_buffer.resize(1, 3, 0.0);
        graph.process(&mut process_buffer);
        let output = process_buffer.slice().to_vec();
        assert_eq!(output, vec![3.0, 3.0, 3.0]);
    }

    #[test]
    fn test_process_sine_generator_in_the_graph_produces_sine() {
        type BufferType = VecAudioBuffer<f32>;
        let settings = AudioProcessorSettings::default();

        let mut oscillator = Oscillator::sine(settings.sample_rate);
        oscillator.set_frequency(440.0);
        let oscillator = OscillatorProcessor { oscillator };

        let reference_sine: BufferType = sine_buffer(
            settings.sample_rate,
            440.0,
            Duration::from_secs_f32((1.0 / settings.sample_rate) * 1000.0),
        )
        .into();

        let mut process_buffer = VecAudioBuffer::new();
        process_buffer.resize(1, 1000, 0.0);

        let mut graph = AudioProcessorGraph::<BufferType>::default();
        graph.prepare(settings);
        let oscillator_idx = graph.add_node(NodeType::Simple(Box::new(oscillator)));
        graph
            .add_connection(graph.input_node, oscillator_idx)
            .unwrap();
        graph
            .add_connection(oscillator_idx, graph.output_node)
            .unwrap();
        graph.process(&mut process_buffer);

        test_level_equivalence(
            reference_sine.slice(),
            process_buffer.slice(),
            1000,
            1000,
            f32::EPSILON,
        );
    }
}

#[derive(Clone)]
pub struct OscillatorProcessor {
    pub oscillator: Oscillator<f32>,
}

impl SimpleAudioProcessor for OscillatorProcessor {
    type SampleType = f32;

    fn s_prepare(&mut self, settings: AudioProcessorSettings) {
        self.oscillator.set_sample_rate(settings.sample_rate);
    }

    fn s_process(&mut self, _sample: Self::SampleType) -> Self::SampleType {
        self.oscillator.next_sample()
    }
}
