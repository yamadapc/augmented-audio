use std::time::Duration;

use audio_garbage_collector::Shared;
use audio_processor_graph::{AudioProcessorGraph, NodeType};
use audio_processor_time::{MonoDelayProcessor, MonoDelayProcessorHandle};
use augmented_dsp_filters::rbj::{FilterProcessor, FilterType};

fn main() {
    type GraphType = AudioProcessorGraph;

    let mut graph: GraphType = AudioProcessorGraph::default();

    for i in 0..10 {
        let i = (i + 1) as f32;
        let delay = MonoDelayProcessor::new(
            Duration::from_secs(6),
            Shared::new(
                audio_garbage_collector::handle(),
                MonoDelayProcessorHandle::default(),
            ),
        );
        delay.handle().set_delay_time_secs(2.0 / i);
        delay.handle().set_feedback(0.2);
        let delay = graph.add_node(NodeType::Simple(Box::new(delay)));

        let mut low_pass_filter = FilterProcessor::new(FilterType::LowPass);
        low_pass_filter.set_cutoff(1500.0 / i);
        low_pass_filter.set_q(2.0);
        let low_pass_filter = graph.add_node(NodeType::Simple(Box::new(low_pass_filter)));

        graph.add_connection(graph.input(), delay).unwrap();
        graph.add_connection(delay, low_pass_filter).unwrap();
        graph
            .add_connection(low_pass_filter, graph.output())
            .unwrap();
    }

    audio_processor_standalone::audio_processor_main(graph);
}
