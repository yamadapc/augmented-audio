use audio_garbage_collector::{make_shared, make_shared_cell, Shared, SharedCell};
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, NoopAudioProcessor,
};
use concread::hashmap::HashMap;
use daggy::Walker;
use std::cell::UnsafeCell;
use std::ops::Deref;
use thiserror::Error;

pub type NodeIndex = daggy::NodeIndex<u32>;
pub type ConnectionIndex = daggy::EdgeIndex<u32>;

struct BufferCell<BufferType>(UnsafeCell<BufferType>);
unsafe impl<BufferType> Sync for BufferCell<BufferType> {}

struct ProcessorCell<SampleType>(UnsafeCell<NodeType<SampleType>>);
unsafe impl<SampleType> Sync for ProcessorCell<SampleType> {}

fn make_processor_cell<SampleType: 'static>(
    processor: NodeType<SampleType>,
) -> Shared<ProcessorCell<SampleType>> {
    make_shared(ProcessorCell(UnsafeCell::new(processor)))
}

pub struct AudioProcessorGraphHandle<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: 'static,
{
    dag: SharedCell<daggy::Dag<(), ()>>,
    process_order: SharedCell<Vec<NodeIndex>>,
    processors: HashMap<NodeIndex, Shared<ProcessorCell<BufferType::SampleType>>>,
    buffers: HashMap<ConnectionIndex, Shared<BufferCell<BufferType>>>,
}

impl<BufferType> AudioProcessorGraphHandle<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: 'static,
{
    pub fn add_node(&self, processor: NodeType<BufferType::SampleType>) -> NodeIndex {
        let mut tx = self.processors.write();
        let mut dag = self.dag.get().deref().clone();
        let index = dag.add_node(());

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

        // TODO - resize the buffer
        let buffer = BufferType::new();
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

type NodeType<SampleType> = Box<dyn SimpleAudioProcessor<SampleType = SampleType> + Send>;

pub struct AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: 'static,
{
    input_node: NodeIndex,
    output_node: NodeIndex,
    handle: Shared<AudioProcessorGraphHandle<BufferType>>,
}

impl<BufferType> Default for AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: Copy + Send + 'static,
{
    fn default() -> Self {
        Self::new(daggy::Dag::default())
    }
}

impl<BufferType> AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer + Send + 'static,
    BufferType::SampleType: Copy + Send + 'static,
{
    fn new(mut dag: daggy::Dag<(), ()>) -> Self {
        let input_proc: NodeType<BufferType::SampleType> = Box::new(NoopAudioProcessor::default());
        let input_node = dag.add_node(());
        let output_proc: NodeType<BufferType::SampleType> = Box::new(NoopAudioProcessor::default());
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
                processors: HashMap::new(),
                buffers: HashMap::new(),
            }),
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

    pub fn add_node(&mut self, processor: NodeType<BufferType::SampleType>) -> NodeIndex {
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
    BufferType::SampleType: Copy,
{
    type SampleType = BufferType::SampleType;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
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
                    (*processor.deref().0.get()).s_prepare(settings);
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

        for node_index in process_order {
            let inputs = dag.parents(*node_index);
            for (connection_id, _) in inputs.iter(dag) {
                if let Some(buffer_ref) = dag
                    .edge_weight(connection_id)
                    .and(buffers.get(&connection_id))
                {
                    let buffer = buffer_ref.deref().0.get();
                    unsafe {
                        copy_buffer(&*buffer, data);
                    }
                }
            }

            if let Some(processor_ref) =
                dag.node_weight(*node_index).and(processors.get(node_index))
            {
                let processor = processor_ref.deref().0.get();

                for frame in data.frames_mut() {
                    unsafe {
                        (*processor).s_process_frame(frame);
                    }
                }
            }

            let mut outputs = dag.children(*node_index);
            while let Some((connection_id, _)) = outputs.walk_next(&dag) {
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
    use audio_processor_traits::audio_buffer::VecAudioBuffer;
    use audio_processor_utility::gain::GainProcessor;
    use audio_processor_utility::pan::PanProcessor;

    use super::*;

    #[test]
    fn test_create_graph() {
        let _ = AudioProcessorGraph::<VecAudioBuffer<f32>>::default();
    }

    #[test]
    fn test_create_graph_and_add_node() {
        let mut graph = AudioProcessorGraph::<VecAudioBuffer<f32>>::default();
        graph.add_node(Box::new(GainProcessor::default()));
    }

    #[test]
    fn test_create_graph_and_add_2_nodes() {
        let mut graph = AudioProcessorGraph::<VecAudioBuffer<f32>>::default();
        let gain1 = graph.add_node(Box::new(GainProcessor::default()));
        let gain2 = graph.add_node(Box::new(GainProcessor::default()));
        let _connection_id = graph.add_connection(gain1, gain2).unwrap();
    }

    #[test]
    fn test_create_heterogeneous_graph() {
        type BufferType = VecAudioBuffer<f32>;

        let mut graph = AudioProcessorGraph::<BufferType>::default();
        let gain = Box::new(GainProcessor::default());
        let gain = graph.add_node(gain);
        let pan = graph.add_node(Box::new(PanProcessor::default()));
        let _connection_id = graph.add_connection(gain, pan).unwrap();
    }
}
