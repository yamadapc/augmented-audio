use audio_processor_utility::pan::PanProcessor;

fn main() {
    let mut pan = PanProcessor::default();
    pan.set_panning(0.6);
    audio_processor_standalone::audio_processor_main(pan);
}
