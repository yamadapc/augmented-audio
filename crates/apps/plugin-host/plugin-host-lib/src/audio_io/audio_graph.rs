// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use actix::{Actor, Context, Handler, Message, MessageResult, Supervised, SystemService};

use audio_garbage_collector::Shared;
use audio_processor_graph::{
    AudioProcessorGraph, AudioProcessorGraphHandle, DefaultProcessor, NodeIndex, NodeType,
};

use audio_processor_traits::{AudioContext, AudioProcessor};

use crate::audio_io::audio_thread;
use crate::audio_io::audio_thread::{AudioThread, AudioThreadProcessor};
use crate::processors::shared_processor::SharedProcessor;

/// Alternate manager for the audio-thread processors using `audio-processor-graph`.
#[derive(Default)]
pub struct AudioGraphManager {
    input_idx: Option<NodeIndex>,
    output_idx: Option<NodeIndex>,
    graph_handle: Option<Shared<AudioProcessorGraphHandle>>,
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

        let mut audio_graph_processor = AudioProcessorGraph::default();
        // TODO: This is wrong. The graph should negotiate settings with the audio-thread.
        let settings = AudioThread::default_settings().unwrap();
        let mut context = AudioContext::from(settings);
        audio_graph_processor.prepare(&mut context, settings);

        self.input_idx = Some(audio_graph_processor.input());
        self.output_idx = Some(audio_graph_processor.output());
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
    RawProcessor { value: NodeType<DefaultProcessor> },
}

#[derive(Message)]
#[rtype(result = "Option<NodeIndex>")]
pub struct CreateAudioNodeMessage {
    pub processor_spec: ProcessorSpec,
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

#[derive(Message)]
#[rtype(result = "Option<(NodeIndex, NodeIndex)>")]
pub struct GetSystemIndexesMessage;

impl Handler<GetSystemIndexesMessage> for AudioGraphManager {
    type Result = MessageResult<GetSystemIndexesMessage>;

    fn handle(&mut self, _msg: GetSystemIndexesMessage, _ctx: &mut Self::Context) -> Self::Result {
        let result = self
            .input_idx
            .and_then(|idx| self.output_idx.map(|oidx| (idx, oidx)));
        MessageResult(result)
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectMessage {
    pub input_index: NodeIndex,
    pub output_index: NodeIndex,
}

impl Handler<ConnectMessage> for AudioGraphManager {
    type Result = ();

    fn handle(&mut self, msg: ConnectMessage, _ctx: &mut Self::Context) -> Self::Result {
        log::info!(
            "Adding connection input={:?} output={:?}",
            msg.input_index,
            msg.output_index
        );
        self.graph_handle
            .as_ref()
            .map(|graph_handle| graph_handle.add_connection(msg.input_index, msg.output_index));
    }
}
