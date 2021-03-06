// = copyright ====================================================================
// DAW: Flutter UI for a DAW application
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
use audio_processor_graph::NodeType;
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
        audio_node_create_raw(NodeType::Simple(Box::new(StereoToMonoProcessor::default()))) as u32;
    let mono_to_stereo_idx =
        audio_node_create_raw(NodeType::Simple(Box::new(MonoToStereoProcessor::default()))) as u32;

    let rms_processor = RunningRMSProcessor::new_with_duration(
        audio_garbage_collector::handle(),
        Duration::from_millis(13),
    );
    let rms_handle = rms_processor.handle().clone();
    let rms_processor_idx = audio_node_create_raw(NodeType::Simple(Box::new(rms_processor))) as u32;
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
