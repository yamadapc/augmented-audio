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
use audio_processor_graph::{AudioProcessorGraph, NodeType};
use audio_processor_traits::{AudioProcessor, BufferProcessor, SliceAudioProcessor};
use audio_processor_utility::gain::GainProcessor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("AudioProcessorGraph::process");

    group.bench_function("WITHOUT GRAPH - GainProcessor::process_slice", |b| {
        let processor: GainProcessor<f32> = GainProcessor::default();
        processor.set_gain(0.3);
        let mut processor = BufferProcessor(processor);
        let mut buffer =
            audio_processor_traits::audio_buffer::VecAudioBuffer::empty_with(1, 1000, 1.0);
        b.iter(|| {
            processor.process(&mut buffer);
            black_box(&mut buffer);
            black_box(&mut processor);
        });
    });

    group.bench_function("WITH STATIC GRAPH - GainProcessor::process_slice", |b| {
        let processor = GainProcessor::default();
        processor.set_gain(0.3);
        let processor = BufferProcessor(processor);

        let mut graph = audio_processor_graph::AudioProcessorGraphImpl::default();
        let node_idx = graph.add_node(NodeType::Static(processor));
        graph.add_connection(graph.input(), node_idx).unwrap();
        graph.add_connection(node_idx, graph.output()).unwrap();

        let mut buffer =
            audio_processor_traits::audio_buffer::VecAudioBuffer::empty_with(1, 1000, 1.0);
        b.iter(|| {
            graph.process(&mut buffer);
            black_box(&mut buffer);
            black_box(&mut graph);
        });
    });

    group.bench_function("WITH GRAPH - GainProcessor::process_slice", |b| {
        let processor = GainProcessor::default();
        processor.set_gain(0.3);
        let processor: Box<dyn SliceAudioProcessor + Send> = Box::new(BufferProcessor(processor));

        let mut graph = AudioProcessorGraph::default();
        let node_idx = graph.add_node(NodeType::Buffer(processor));
        graph.add_connection(graph.input(), node_idx).unwrap();
        graph.add_connection(node_idx, graph.output()).unwrap();

        let mut buffer =
            audio_processor_traits::audio_buffer::VecAudioBuffer::empty_with(1, 1000, 1.0);
        b.iter(|| {
            graph.process(&mut buffer);
            black_box(&mut buffer);
            black_box(&mut graph);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
