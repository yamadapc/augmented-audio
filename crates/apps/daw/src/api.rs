use std::path::PathBuf;
use std::time::Duration;

use actix::{Handler, SystemService};
use anyhow::Result;
use flutter_rust_bridge::StreamSink;

use audio_garbage_collector::Shared;
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
        // TODO - handle mailbox error
        let _ = host_addr.send(msg).await;
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
    let audio_file_processor: Shared<AudioFileProcessorHandle> = ProcessorHandleRegistry::current()
        .get("audio-file")
        .unwrap();
    audio_file_processor.play();
    Ok(0)
}

pub fn stop_playback() -> Result<i32> {
    let audio_file_processor: Shared<AudioFileProcessorHandle> = ProcessorHandleRegistry::current()
        .get("audio-file")
        .unwrap();
    audio_file_processor.stop();
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
