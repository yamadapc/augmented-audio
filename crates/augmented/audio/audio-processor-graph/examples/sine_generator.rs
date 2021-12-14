


use audio_processor_graph::{AudioProcessorGraph, OscillatorProcessor};

use audio_processor_traits::audio_buffer::VecAudioBuffer;


fn main() {
    type BufferType = VecAudioBuffer<f32>;
    type GraphType = AudioProcessorGraph<BufferType>;

    let mut graph: GraphType = AudioProcessorGraph::default();

    let mut oscillator = oscillator::Oscillator::sine(44100.0);
    oscillator.set_frequency(440.0);
    let oscillator = OscillatorProcessor { oscillator };
    let oscillator_idx = graph.add_node(Box::new(oscillator));
    graph.add_connection(graph.input(), oscillator_idx).unwrap();
    graph
        .add_connection(oscillator_idx, graph.output())
        .unwrap();

    audio_processor_standalone::audio_processor_main(graph);
}
