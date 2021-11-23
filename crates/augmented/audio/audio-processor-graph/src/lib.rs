use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, NoopAudioProcessor,
};
use connection::Connection;
use daggy::Walker;
use thiserror::Error;

mod connection;

pub type NodeIndex = daggy::NodeIndex<u32>;
pub type ConnectionIndex = daggy::EdgeIndex<u32>;

#[derive(Debug, Error)]
pub enum AudioProcessorGraphError {
    #[error("Adding this connection would result in a cycle")]
    WouldCycle,
}

type NodeType<SampleType> = Box<dyn SimpleAudioProcessor<SampleType = SampleType> + Send>;

pub struct AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer,
{
    input_node: NodeIndex,
    output_node: NodeIndex,
    dag: daggy::Dag<NodeType<BufferType::SampleType>, Connection<BufferType>>,
    process_order: Vec<NodeIndex>,
}

impl<BufferType> Default for AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer,
    BufferType::SampleType: Copy + Send + 'static,
{
    fn default() -> Self {
        Self::new(daggy::Dag::default())
    }
}

impl<BufferType> AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer,
    BufferType::SampleType: Copy + Send + 'static,
{
    pub fn new(
        mut dag: daggy::Dag<NodeType<BufferType::SampleType>, Connection<BufferType>>,
    ) -> Self {
        let input_node = dag.add_node(Box::new(NoopAudioProcessor::default()));
        let output_node = dag.add_node(Box::new(NoopAudioProcessor::default()));

        AudioProcessorGraph {
            input_node,
            output_node,
            dag,
            process_order: Vec::new(),
        }
    }

    pub fn input(&self) -> NodeIndex {
        self.input_node
    }

    pub fn output(&self) -> NodeIndex {
        self.output_node
    }

    pub fn add_node(&mut self, processor: NodeType<BufferType::SampleType>) -> NodeIndex {
        self.dag.add_node(processor)
    }

    pub fn add_connection(
        &mut self,
        source: NodeIndex,
        destination: NodeIndex,
    ) -> Result<ConnectionIndex, AudioProcessorGraphError> {
        let edge = self
            .dag
            .add_edge(source, destination, Connection::new())
            .map_err(|_| AudioProcessorGraphError::WouldCycle)?;

        self.prepare_order()
            .map_err(|_| AudioProcessorGraphError::WouldCycle)?;

        Ok(edge)
    }

    fn prepare_order(&mut self) -> Result<(), daggy::petgraph::algo::Cycle<daggy::NodeIndex>> {
        self.process_order = daggy::petgraph::algo::toposort(&self.dag, None)?;
        Ok(())
    }
}

impl<BufferType> AudioProcessor for AudioProcessorGraph<BufferType>
where
    BufferType: OwnedAudioBuffer,
    BufferType::SampleType: Copy,
{
    type SampleType = BufferType::SampleType;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        let process_order = &self.process_order;
        for node_index in process_order {
            if let Some(processor) = self.dag.node_weight_mut(*node_index) {
                processor.s_prepare(settings);
            }
        }
    }

    fn process<InputBufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut InputBufferType,
    ) {
        let process_order = &self.process_order;

        for node_index in process_order {
            let inputs = self.dag.parents(*node_index);
            for (connection_id, _) in inputs.iter(&self.dag) {
                if let Some(connection_buffer) = self.dag.edge_weight(connection_id) {
                    copy_buffer(connection_buffer.buffer(), data);
                }
            }

            if let Some(processor) = self.dag.node_weight_mut(*node_index) {
                for frame in data.frames_mut() {
                    processor.s_process_frame(frame);
                }
            }

            let mut outputs = self.dag.children(*node_index);
            while let Some((connection_id, _)) = outputs.walk_next(&self.dag) {
                if let Some(connection_buffer) = self.dag.edge_weight_mut(connection_id) {
                    copy_buffer(data, connection_buffer.buffer_mut());
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
