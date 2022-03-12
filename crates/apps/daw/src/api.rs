use std::path::PathBuf;
use std::time::Duration;

use actix::{Handler, SystemService};
use anyhow::Result;
use flutter_rust_bridge::StreamSink;

use audio_garbage_collector::Shared;
use audio_processor_graph::{NodeIndex, NodeType};

use audio_thread::actor::AudioThreadMessage;
use plugin_host_lib::actor_system::ActorSystemThread;
use plugin_host_lib::audio_io::audio_graph;
use plugin_host_lib::audio_io::audio_graph::AudioGraphManager;
use plugin_host_lib::audio_io::audio_thread;
use plugin_host_lib::audio_io::audio_thread::options::{AudioDeviceId, AudioHostId};
use plugin_host_lib::audio_io::audio_thread::AudioThread;
use plugin_host_lib::audio_io::processor_handle_registry::ProcessorHandleRegistry;
use plugin_host_lib::audio_io::{
    AudioIOService, LoadPluginMessage, SetAudioFilePathMessage, StartMessage,
};
use plugin_host_lib::processors::audio_file_processor::AudioFileProcessorHandle;
use plugin_host_lib::TestPluginHost;

fn send_host_message<M>(msg: M)
where
    M: actix::Message + Send + 'static,
    M::Result: Send,
    TestPluginHost: Handler<M>,
{
    let actor_system_thread = plugin_host_lib::actor_system::ActorSystemThread::current();
    actor_system_thread.spawn(async move {
        let host_addr = TestPluginHost::from_registry();
        host_addr.send(msg).await.unwrap();
    });
}

pub fn initialize_logger() -> Result<i32> {
    let _ = wisual_logger::try_init_from_env();
    log::info!("Rust logger initialized");
    Ok(0)
}

pub fn initialize_audio() -> Result<i32> {
    send_host_message(StartMessage);
    Ok(0)
}

pub fn start_playback() -> Result<i32> {
    if let Some(audio_file_processor) =
        ProcessorHandleRegistry::current().get::<Shared<AudioFileProcessorHandle>>("audio-file")
    {
        audio_file_processor.play();
    }
    Ok(0)
}

pub fn stop_playback() -> Result<i32> {
    if let Some(audio_file_processor) =
        ProcessorHandleRegistry::current().get::<Shared<AudioFileProcessorHandle>>("audio-file")
    {
        audio_file_processor.stop();
    }
    Ok(0)
}

pub fn set_vst_file_path(path: String) -> Result<i32> {
    send_host_message(LoadPluginMessage {
        plugin_path: PathBuf::from(path),
    });
    Ok(0)
}

pub fn set_input_file_path(path: String) -> Result<i32> {
    send_host_message(SetAudioFilePathMessage(PathBuf::from(path)));
    Ok(0)
}

pub fn audio_io_get_input_devices() -> Result<String> {
    let devices_list = AudioIOService::devices_list(None)?;
    let result = serde_json::to_string(&devices_list)?;
    Ok(result)
}

pub fn get_events_sink(sink: StreamSink<String>) -> Result<i32> {
    std::thread::spawn(move || loop {
        sink.add("MESSAGE".to_string());
        std::thread::sleep(Duration::from_millis(1000));
    });
    Ok(0)
}

pub fn audio_thread_set_options(input_device_id: String, output_device_id: String) -> Result<i32> {
    let actor_system_thread = ActorSystemThread::current();
    actor_system_thread.spawn_result(async move {
        let audio_thread = AudioThread::from_registry();
        audio_thread
            .send(AudioThreadMessage::SetOptions {
                host_id: AudioHostId::Default,
                input_device_id: if input_device_id == "default" {
                    Some(AudioDeviceId::Default)
                } else {
                    Some(AudioDeviceId::Id(input_device_id))
                },
                output_device_id: if output_device_id == "default" {
                    AudioDeviceId::Default
                } else {
                    AudioDeviceId::Id(output_device_id)
                },
            })
            .await
            .unwrap()
            .unwrap();
    });
    Ok(0)
}

pub fn audio_graph_setup() -> Result<i32> {
    log::info!("Starting audio-graph-manager");
    let actor_system_thread = ActorSystemThread::current();
    actor_system_thread.spawn_result(async move {
        let manager = AudioGraphManager::from_registry();
        manager.send(audio_graph::SetupGraphMessage).await.unwrap();
        let audio_thread = AudioThread::from_registry();
        audio_thread
            .send(AudioThreadMessage::Start)
            .await
            .unwrap()
            .unwrap();
    });
    Ok(0)
}

pub fn audio_graph_get_system_indexes() -> Result<Vec<u32>> {
    ActorSystemThread::current().spawn_result(async move {
        let manager = AudioGraphManager::from_registry();
        let (input_index, output_index) = manager
            .send(audio_graph::GetSystemIndexesMessage)
            .await
            .unwrap()
            .unwrap();
        Ok(vec![
            input_index.index() as u32,
            output_index.index() as u32,
        ])
    })
}

pub fn audio_graph_connect(input_index: u32, output_index: u32) -> Result<u32> {
    ActorSystemThread::current().spawn(async move {
        let manager = AudioGraphManager::from_registry();
        manager
            .send(audio_graph::ConnectMessage {
                input_index: NodeIndex::new(input_index as usize),
                output_index: NodeIndex::new(output_index as usize),
            })
            .await
            .unwrap();
    });
    Ok(0)
}

pub fn audio_node_create(audio_processor_name: String) -> Result<u32> {
    let processor: Result<NodeType<f32>> = match audio_processor_name.as_str() {
        "delay" => Ok(Box::new(audio_processor_time::MonoDelayProcessor::default())),
        "filter" => Ok(Box::new(augmented_dsp_filters::rbj::FilterProcessor::new(
            augmented_dsp_filters::rbj::FilterType::LowPass,
        ))),
        "gain" => Ok(Box::new(
            audio_processor_utility::gain::GainProcessor::default(),
        )),
        "pan" => Ok(Box::new(
            audio_processor_utility::pan::PanProcessor::default(),
        )),
        _ => Err(anyhow::Error::msg("Failed to create processor")),
    };
    let processor = processor?;

    let index = crate::graph::audio_node_create_raw(processor);

    Ok(index as u32)
}

pub fn audio_node_set_parameter(
    _audio_node_id: i32,
    _parameter_name: String,
    _parameter_value: f32,
) -> Result<i32> {
    todo!()
}
