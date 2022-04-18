use augmented_midi::ParserState;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse MIDI message");

    group.bench_function("augmented_midi::parse_midi_event", |b| {
        let input_buffer = [0x9_8, 0x3C, 0x44];
        let mut parser_state = ParserState::default();
        b.iter(|| {
            let mut output =
                augmented_midi::parse_midi_event::<&[u8]>(&input_buffer, &mut parser_state);
            black_box(&mut output);
        });
    });

    group.bench_function(
        "augmented_midi::parse_midi_event - owned buffer (more similar to rimd)",
        |b| {
            let input_buffer = [0x9_8, 0x3C, 0x44];
            let mut parser_state = ParserState::default();
            b.iter(|| {
                let mut output =
                    augmented_midi::parse_midi_event::<Vec<u8>>(&input_buffer, &mut parser_state);
                black_box(&mut output);
            });
        },
    );

    group.bench_function(
        "rimd::MidiMessage::from_bytes - considering input allocation",
        |b| {
            b.iter(|| {
                let input_buffer = vec![0x9_8, 0x3C, 0x44];
                let mut output = rimd::MidiMessage::from_bytes(input_buffer);
                black_box(&mut output);
            });
        },
    );

    group.bench_function(
        "rimd::MidiMessage::from_bytes - ignoring input allocation",
        |b| {
            b.iter_batched(
                || vec![0x9_8, 0x3C, 0x44],
                |input_buffer| {
                    let mut output = rimd::MidiMessage::from_bytes(input_buffer);
                    black_box(&mut output);
                },
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
