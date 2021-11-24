use actix::{Actor, Context, Handler, Message, MessageResult, Supervised, SystemService};
use audio_garbage_collector::Shared;

use audio_processor_graph::{AudioProcessorGraph, AudioProcessorGraphHandle, NodeIndex, NodeType};
use audio_processor_traits::audio_buffer::VecAudioBuffer;

use crate::audio_io::audio_thread;
use crate::audio_io::audio_thread::{AudioThread, AudioThreadProcessor};
use crate::processors::shared_processor::SharedProcessor;

/// Alternate manager for the audio-thread processors using `audio-processor-graph`.
pub struct AudioGraphManager {
    graph_handle: Option<Shared<AudioProcessorGraphHandle<VecAudioBuffer<f32>>>>,
}

impl Default for AudioGraphManager {
    fn default() -> Self {
        AudioGraphManager { graph_handle: None }
    }
}

impl Actor for AudioGraphManager {
    type Context = Context<Self>;
}

impl Supervised for AudioGraphManager {}

impl SystemService for AudioGraphManager {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        log::info!("AudioGraphManager started");
    }
}

/// Will force the `AudioThread` to use a graph processor.
#[derive(Message)]
#[rtype(result = "()")]
pub struct SetupGraphMessage;

impl Handler<SetupGraphMessage> for AudioGraphManager {
    type Result = ();

    fn handle(&mut self, _msg: SetupGraphMessage, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Setting-up audio-graph processor");

        let audio_graph_processor = AudioProcessorGraph::default();
        self.graph_handle = Some(audio_graph_processor.handle().clone());

        let processor = SharedProcessor::new(
            audio_garbage_collector::handle(),
            AudioThreadProcessor::Graph(audio_graph_processor),
        );
        let audio_thread = AudioThread::from_registry();
        audio_thread.do_send(audio_thread::actor::AudioThreadMessage::SetProcessor { processor })
    }
}

pub enum ProcessorSpec {
    RawProcessor { value: NodeType<f32> },
}

#[derive(Message)]
#[rtype(result = "Option<NodeIndex>")]
pub struct CreateAudioNodeMessage {
    processor_spec: ProcessorSpec,
}

impl Handler<CreateAudioNodeMessage> for AudioGraphManager {
    type Result = MessageResult<CreateAudioNodeMessage>;

    fn handle(&mut self, msg: CreateAudioNodeMessage, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.graph_handle.as_ref().map(|graph_handle| {
            let processor = match msg.processor_spec {
                ProcessorSpec::RawProcessor { value } => value,
            };
            let index = graph_handle.add_node(processor);
            log::info!("Adding audio node index={:?}", index);
            index
        }))
    }
}
