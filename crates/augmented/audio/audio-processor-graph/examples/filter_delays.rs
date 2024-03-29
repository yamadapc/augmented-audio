// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::time::Duration;

use audio_garbage_collector::Shared;
use audio_processor_graph::{AudioProcessorGraph, NodeType};
use audio_processor_time::{MonoDelayProcessor, MonoDelayProcessorHandle};
use audio_processor_traits::simple_processor::MonoCopyProcessor;
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
        let delay = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(delay))));

        let mut low_pass_filter = FilterProcessor::new(FilterType::LowPass);
        low_pass_filter.set_cutoff(1500.0 / i);
        low_pass_filter.set_q(2.0);
        let low_pass_filter = graph.add_node(NodeType::Simple(Box::new(MonoCopyProcessor::new(
            low_pass_filter,
        ))));

        graph.add_connection(graph.input(), delay).unwrap();
        graph.add_connection(delay, low_pass_filter).unwrap();
        graph
            .add_connection(low_pass_filter, graph.output())
            .unwrap();
    }

    audio_processor_standalone::audio_processor_main(graph);
}
