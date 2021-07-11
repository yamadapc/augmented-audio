use audio_processor_traits::ObjectAudioProcessor;
use std::marker::PhantomData;

type NodeIndex = daggy::NodeIndex<u32>;

struct AudioProcessorGraph<BufferType, Processor, EdgeWeight> {
    dag: daggy::Dag<Processor, EdgeWeight>,
    phantom: PhantomData<BufferType>,
}

impl<BufferType, Processor, EdgeWeight> Default
    for AudioProcessorGraph<BufferType, Processor, EdgeWeight>
{
    fn default() -> Self {
        Self::new(daggy::Dag::default())
    }
}

impl<BufferType, Processor, EdgeWeight> AudioProcessorGraph<BufferType, Processor, EdgeWeight> {
    pub fn new(dag: daggy::Dag<Processor, EdgeWeight>) -> Self {
        AudioProcessorGraph {
            dag,
            phantom: PhantomData::default(),
        }
    }

    pub fn add_node(&mut self, processor: Processor) -> NodeIndex {
        self.dag.add_node(processor)
    }
}

#[cfg(test)]
mod test {
    use audio_processor_traits::{InterleavedAudioBuffer, ObjectAudioProcessor};
    use audio_processor_utility::gain::GainProcessor;

    use super::*;

    #[test]
    fn test_create_graph() {
        let _ = AudioProcessorGraph::<f32, GainProcessor<f32>, ()>::default();
    }

    #[test]
    fn test_create_graph_and_add_node() {
        let mut graph = AudioProcessorGraph::<
            f32,
            Box<dyn ObjectAudioProcessor<InterleavedAudioBuffer<f32>>>,
            (),
        >::default();
        graph.add_node(Box::new(GainProcessor::default()));
    }
}
