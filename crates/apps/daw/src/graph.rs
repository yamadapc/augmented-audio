use actix::SystemService;
use audio_processor_graph::{NodeType};
use plugin_host_lib::actor_system::ActorSystemThread;
use plugin_host_lib::audio_io::audio_graph;
use plugin_host_lib::audio_io::audio_graph::{AudioGraphManager, ProcessorSpec};

pub fn audio_node_create_raw(processor: NodeType<f32>) -> usize {
    let index = ActorSystemThread::current().spawn_result(async move {
        let manager = AudioGraphManager::from_registry();
        manager
            .send(audio_graph::CreateAudioNodeMessage {
                processor_spec: ProcessorSpec::RawProcessor { value: processor },
            })
            .await
            .unwrap()
            .unwrap()
            .index()
    });
    index
}

