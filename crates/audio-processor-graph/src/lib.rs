use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::ObjectAudioProcessor;
use connection::Connection;

mod connection;

type NodeIndex = daggy::NodeIndex<u32>;
type ConnectionIndex = daggy::EdgeIndex<u32>;

struct AudioProcessorGraph<BufferType, Processor>
where
    BufferType: OwnedAudioBuffer,
    Processor: ObjectAudioProcessor<BufferType>,
{
    dag: daggy::Dag<Processor, Connection<BufferType>>,
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
        AudioProcessorGraph { dag }
    }

    pub fn add_node(&mut self, processor: Processor) -> NodeIndex {
        self.dag.add_node(processor)
    }

    pub fn add_connection(
        &mut self,
        source: NodeIndex,
        destination: NodeIndex,
    ) -> Result<ConnectionIndex, daggy::WouldCycle<Connection<BufferType>>> {
        self.dag.add_edge(source, destination, Connection::new())
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
        graph.add_connection(gain1, gain2).unwrap();
    }
}
