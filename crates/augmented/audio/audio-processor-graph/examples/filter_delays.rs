use audio_processor_graph::AudioProcessorGraph;
use audio_processor_time::MonoDelayProcessor;
use audio_processor_traits::audio_buffer::VecAudioBuffer;
use augmented_dsp_filters::rbj::{FilterProcessor, FilterType};

fn main() {
    type BufferType = VecAudioBuffer<f32>;
    type GraphType = AudioProcessorGraph<BufferType>;

    let mut graph: GraphType = AudioProcessorGraph::default();

    for i in 0..10 {
        let i = (i + 1) as f32;
        let delay = MonoDelayProcessor::default();
        delay.handle().set_delay_time_secs(1.0 / i);
        delay.handle().set_feedback(0.6 / i);
        let delay = graph.add_node(Box::new(delay));

        let mut low_pass_filter = FilterProcessor::new(FilterType::LowPass);
        low_pass_filter.set_cutoff(3000.0 / (11.0 - i));
        low_pass_filter.set_q(2.0);
        let low_pass_filter = graph.add_node(Box::new(low_pass_filter));

        graph.add_connection(graph.input(), delay).unwrap();
        graph.add_connection(delay, low_pass_filter).unwrap();
        graph
            .add_connection(low_pass_filter, graph.output())
            .unwrap();
    }

    audio_processor_standalone::audio_processor_main(graph);
}
