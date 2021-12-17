use audio_processor_utility::mono::StereoToMonoProcessor;
use audio_processor_utility::stereo::MonoToStereoProcessor;
use std::time::Duration;

use daw_ui::api::{
    audio_graph_connect, audio_graph_get_system_indexes, audio_graph_setup, audio_node_create,
    audio_thread_set_options,
};
use daw_ui::graph::audio_node_create_raw;
use plugin_host_lib::processors::running_rms_processor::RunningRMSProcessor;

fn main() {
    wisual_logger::init_from_env();

    audio_graph_setup().unwrap();
    audio_thread_set_options("default".into(), "default".into()).unwrap();
    let result = audio_graph_get_system_indexes().unwrap();
    let input_idx = result[0];
    let output_idx = result[1];
    let delay_idx = audio_node_create("delay".into()).unwrap();

    let stereo_to_mono_idx =
        audio_node_create_raw(Box::new(StereoToMonoProcessor::default())) as u32;
    let mono_to_stereo_idx =
        audio_node_create_raw(Box::new(MonoToStereoProcessor::default())) as u32;

    let rms_processor = RunningRMSProcessor::new_with_duration(
        audio_garbage_collector::handle(),
        Duration::from_millis(13),
    );
    let rms_handle = rms_processor.handle().clone();
    let rms_processor_idx = audio_node_create_raw(Box::new(rms_processor)) as u32;
    audio_graph_connect(input_idx, stereo_to_mono_idx).unwrap();
    audio_graph_connect(stereo_to_mono_idx, delay_idx).unwrap();
    audio_graph_connect(stereo_to_mono_idx, mono_to_stereo_idx).unwrap();
    audio_graph_connect(delay_idx, mono_to_stereo_idx).unwrap();
    audio_graph_connect(mono_to_stereo_idx, rms_processor_idx).unwrap();
    audio_graph_connect(rms_processor_idx, output_idx).unwrap();

    loop {
        log::info!(
            "Current RMS {} / {}",
            rms_handle.calculate_rms(0),
            rms_handle.calculate_rms(1)
        );
        std::thread::sleep(Duration::from_secs(1));
    }
}
