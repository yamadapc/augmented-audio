use audio_processor_graph::AudioProcessorGraph;
use audio_processor_traits::audio_buffer::VecAudioBuffer;

fn main() {
    type BufferType = VecAudioBuffer<f32>;
    type GraphType = AudioProcessorGraph;

    let mut graph: GraphType = AudioProcessorGraph::default();
    graph.add_connection(graph.input(), graph.output()).unwrap();
    audio_processor_standalone::audio_processor_main(graph);
}
