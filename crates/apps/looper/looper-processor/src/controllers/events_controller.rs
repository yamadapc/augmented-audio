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

//! This is a global application events bus.
//!
//! When an application event happens it's broadcasted by this controller.
//!
//! It is useful for broadcasting events from the engine into the GUI.
use actix::{Actor, Handler};

use crate::common::Consumer;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum ApplicationEvent {
    ApplicationEventLooperClipUpdated {
        looper_id: usize,
    },
    #[cfg(test)]
    TestPing,
}

type BoxedConsumer = Box<dyn Consumer<ApplicationEvent> + Send + 'static>;

#[derive(Default)]
pub struct EventsController {
    consumers: Vec<BoxedConsumer>,
}

impl Actor for EventsController {
    type Context = actix::Context<Self>;
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct AddConsumerMessage(pub BoxedConsumer);

impl Handler<AddConsumerMessage> for EventsController {
    type Result = ();

    fn handle(&mut self, msg: AddConsumerMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.consumers.push(msg.0)
    }
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub ApplicationEvent);

impl Handler<BroadcastMessage> for EventsController {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Broadcasting event: {:?}", msg.0);
        for consumer in &self.consumers {
            consumer.accept(msg.0.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;

    use actix::Actor;

    use crate::common::ClosureConsumer;

    use super::*;

    #[actix::test]
    async fn test_events_controller_add_consumer() {
        wisual_logger::init_from_env();
        let controller = EventsController::default().start();
        let mock_consumer = ClosureConsumer::new(|_event| {});
        let mock_consumer: Box<dyn Consumer<ApplicationEvent> + Send + 'static> =
            Box::new(mock_consumer);
        let message = AddConsumerMessage(mock_consumer);
        controller.send(message).await.unwrap();
    }

    #[actix::test]
    async fn test_events_controller_broadcast() {
        wisual_logger::init_from_env();
        let controller = EventsController::default().start();
        let (tx, rx) = channel();
        let mock_consumer = ClosureConsumer::new(move |event| tx.send(event).unwrap());
        let mock_consumer: Box<dyn Consumer<ApplicationEvent> + Send + 'static> =
            Box::new(mock_consumer);
        let message = AddConsumerMessage(mock_consumer);
        controller.send(message).await.unwrap();
        let message = BroadcastMessage(ApplicationEvent::TestPing);
        controller.send(message).await.unwrap();
        let event = rx.recv().unwrap();
        assert_eq!(event, ApplicationEvent::TestPing);
    }
}
