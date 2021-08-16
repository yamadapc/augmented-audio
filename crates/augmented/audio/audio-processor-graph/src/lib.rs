use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, ObjectAudioProcessor};
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

pub struct AudioProcessorGraph<BufferType, Processor>
where
    BufferType: OwnedAudioBuffer,
    Processor: ObjectAudioProcessor<BufferType>,
{
    dag: daggy::Dag<Processor, Connection<BufferType>>,
    process_order: Vec<NodeIndex>,
}

impl<BufferType, Processor> Default for AudioProcessorGraph<BufferType, Processor>
where
    BufferType: OwnedAudioBuffer,
    Processor: ObjectAudioProcessor<BufferType>,
{
    fn default() -> Self {
        Self::new(daggy::Dag::default())
    }
}

impl<BufferType, Processor> AudioProcessorGraph<BufferType, Processor>
where
    BufferType: OwnedAudioBuffer,
    Processor: ObjectAudioProcessor<BufferType>,
{
    pub fn new(dag: daggy::Dag<Processor, Connection<BufferType>>) -> Self {
        AudioProcessorGraph {
            dag,
            process_order: Vec::new(),
        }
    }

    pub fn add_node(&mut self, processor: Processor) -> NodeIndex {
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

impl<BufferType, Processor> ObjectAudioProcessor<BufferType>
    for AudioProcessorGraph<BufferType, Processor>
where
    BufferType: OwnedAudioBuffer,
    BufferType::SampleType: Copy,
    Processor: ObjectAudioProcessor<BufferType>,
{
    fn prepare_obj(&mut self, settings: AudioProcessorSettings) {
        let process_order = &self.process_order;
        for node_index in process_order {
            if let Some(processor) = self.dag.node_weight_mut(*node_index) {
                processor.prepare_obj(settings);
            }
        }
    }

    fn process_obj(&mut self, data: &mut BufferType) {
        let process_order = &self.process_order;

        for node_index in process_order {
            let inputs = self.dag.parents(*node_index);
            for (connection_id, _) in inputs.iter(&self.dag) {
                if let Some(connection_buffer) = self.dag.edge_weight(connection_id) {
                    copy_buffer(connection_buffer.buffer(), data);
                }
            }

            if let Some(processor) = self.dag.node_weight_mut(*node_index) {
                processor.process_obj(data);
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

fn copy_buffer<BufferType>(source: &BufferType, destination: &mut BufferType)
where
    BufferType: AudioBuffer,
    BufferType::SampleType: Copy,
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

    use super::*;

    #[test]
    fn test_create_graph() {
        let _ = AudioProcessorGraph::<VecAudioBuffer<f32>, GainProcessor<f32>>::default();
    }

    #[test]
    fn test_create_graph_and_add_node() {
        let mut graph = AudioProcessorGraph::<VecAudioBuffer<f32>, GainProcessor<f32>>::default();
        graph.add_node(GainProcessor::default());
    }

    #[test]
    fn test_create_graph_and_add_2_nodes() {
        let mut graph = AudioProcessorGraph::<VecAudioBuffer<f32>, GainProcessor<f32>>::default();
        let gain1 = graph.add_node(GainProcessor::default());
        let gain2 = graph.add_node(GainProcessor::default());
        let _connection_id = graph.add_connection(gain1, gain2).unwrap();
    }
}
