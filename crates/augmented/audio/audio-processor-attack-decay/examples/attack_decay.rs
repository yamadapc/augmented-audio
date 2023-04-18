use audio_processor_attack_decay::AttackDecayProcessor;

fn main() {
    let p = AttackDecayProcessor::default();
    audio_processor_standalone::audio_processor_main(p);
}
