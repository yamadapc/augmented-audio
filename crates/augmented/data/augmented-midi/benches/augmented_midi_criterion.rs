use augmented_midi::{parse_midi_file, ParserState};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("augmented_midi");

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

    let input_path = format!(
        "{}/test-files/c1_4over4_1bar.mid",
        env!("CARGO_MANIFEST_DIR")
    );
    let input_file = std::fs::read(input_path).unwrap();
    group.bench_with_input(
        "augmented_midi::parse_midi_file",
        &input_file,
        |b, input_file| {
            b.iter(|| {
                let result = parse_midi_file::<&str, &[u8]>(input_file).unwrap();
                black_box(result);
            });
        },
    );

    group.bench_with_input("rimd::SMF::from_reader", &input_file, |b, input_file| {
        b.iter_batched(
            || std::io::Cursor::new(input_file),
            |mut cursor| {
                let result = rimd::SMF::from_reader(&mut cursor).unwrap();
                black_box(result);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
