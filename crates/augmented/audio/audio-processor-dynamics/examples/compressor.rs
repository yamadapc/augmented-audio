use audio_processor_dynamics::CompressorProcessor;
use audio_processor_standalone::audio_processor_main;
use augmented_audio_volume::amplitude_to_db;

fn main() {
    let processor = CompressorProcessor::new();
    processor.handle().set_threshold(-30.0);
    processor.handle().set_ratio(10.0);
    processor.handle().set_attack_ms(1.0);
    processor
        .handle()
        .set_make_up_gain(amplitude_to_db(0.25, 1.0));
    audio_processor_main(processor);
}
