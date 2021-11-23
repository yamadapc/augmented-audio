use audio_garbage_collector::Handle;
use audio_processor_traits::audio_buffer::VecAudioBuffer;
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};

use crate::StandaloneProcessor;

/// Render a processor offline into a file
pub fn run_offline_render(
    mut app: impl StandaloneProcessor,
    handle: Option<&Handle>,
    input_path: String,
    output_path: String,
) {
    let _ = wisual_logger::try_init_from_env();

    let handle = handle.unwrap_or_else(|| audio_garbage_collector::handle());

    log::info!(
        "Rendering offline input={} output={}",
        input_path,
        output_path
    );

    let buffer_size = 512;
    let sample_rate = 44100.0;
    let audio_processor_settings = AudioProcessorSettings::new(sample_rate, 2, 2, buffer_size);

    let audio_file_settings = audio_processor_file::AudioFileSettings::from_path(&input_path)
        .expect("Failed to read input file");

    // Set-up input file
    log::info!("Loading input file");
    let mut audio_file_processor = audio_processor_file::AudioFileProcessor::new(
        handle,
        audio_file_settings,
        audio_processor_settings,
    );
    audio_file_processor.prepare(audio_processor_settings);
    let audio_file_buffer = audio_file_processor.buffer();
    let audio_file_total_samples = audio_file_buffer[0].len();

    // Set-up output file
    log::info!("Setting-up output buffers");
    let mut output_file_processor = audio_processor_file::OutputAudioFileProcessor::from_path(
        audio_processor_settings,
        &output_path,
    );
    output_file_processor.prepare(audio_processor_settings);

    // Set-up output buffer
    let block_size = audio_processor_settings.block_size() as usize;
    let total_blocks = audio_file_total_samples / block_size;
    let mut buffer = Vec::new();
    buffer.resize(block_size * audio_processor_settings.input_channels(), 0.0);
    let mut buffer = VecAudioBuffer::new_with(buffer, 2, buffer_size);

    log::info!("Setting-up audio processor");
    app.processor().prepare(audio_processor_settings);

    log::info!("Rendering. total_blocks={}", total_blocks);
    for _block_num in 0..total_blocks {
        for sample in buffer.slice_mut() {
            *sample = 0.0;
        }

        audio_file_processor.process(&mut buffer);
        app.processor().process(&mut buffer);
        output_file_processor.process(buffer.slice_mut());
    }
}
