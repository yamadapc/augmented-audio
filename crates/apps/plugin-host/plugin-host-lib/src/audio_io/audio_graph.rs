use actix::{Actor, Context, Handler, Message, SystemService};

use audio_processor_graph::AudioProcessorGraph;

use crate::audio_io::audio_thread;
use crate::audio_io::audio_thread::{AudioThread, AudioThreadProcessor};
use crate::processors::shared_processor::SharedProcessor;

/// Alternate manager for the audio-thread processors using `audio-processor-graph`.
pub struct AudioGraphManager {}

impl Actor for AudioGraphManager {
    type Context = Context<Self>;
}

/// Will force the `AudioThread` to use a graph processor.
#[derive(Message)]
#[rtype(result = "()")]
pub struct SetupGraphMessage;

impl Handler<SetupGraphMessage> for AudioGraphManager {
    type Result = ();

    fn handle(&mut self, _msg: SetupGraphMessage, _ctx: &mut Self::Context) -> Self::Result {
        let audio_graph_processor = AudioProcessorGraph::default();
        let processor = SharedProcessor::new(
            audio_garbage_collector::handle(),
            AudioThreadProcessor::Graph(audio_graph_processor),
        );
        let audio_thread = AudioThread::from_registry();
        audio_thread.do_send(audio_thread::actor::AudioThreadMessage::SetProcessor { processor })
    }
}
