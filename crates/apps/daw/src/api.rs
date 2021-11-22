use actix::SystemService;
use anyhow::Result;

use plugin_host_lib::audio_io::AudioIOService;
use plugin_host_lib::TestPluginHost;

pub fn initialize_logger() -> Result<i32> {
    let _ = wisual_logger::try_init_from_env();
    log::info!("Rust logger initialized");
    Ok(0)
}

pub fn audio_io_get_input_devices() -> Result<String> {
    let actor_system_thread = plugin_host_lib::actor_system::ActorSystemThread::current();
    let _host_addr =
        actor_system_thread.spawn_result(async move { TestPluginHost::from_registry() });

    let devices_list = AudioIOService::devices_list(None)?;
    let result = serde_json::to_string(&devices_list)?;
    Ok(result)
}
