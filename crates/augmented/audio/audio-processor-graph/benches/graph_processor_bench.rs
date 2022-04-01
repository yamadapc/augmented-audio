use audio_processor_graph::AudioProcessorGraph;
use audio_processor_traits::{AudioProcessor, BufferProcessor, SliceAudioProcessor};
use audio_processor_utility::gain::GainProcessor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("AudioProcessorGraph::process");

    group.bench_function("WITHOUT GRAPH - NoopProcessor::process_slice", |b| {
        let processor = GainProcessor::default();
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

    group.bench_function("WITH GRAPH - NoopProcessor::process_slice", |b| {
        let processor = GainProcessor::default();
        processor.set_gain(0.3);
        let processor: Box<dyn SliceAudioProcessor + Send> = Box::new(BufferProcessor(processor));

        let mut graph = AudioProcessorGraph::default();
        let node_idx = graph.add_node(processor.into());
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
