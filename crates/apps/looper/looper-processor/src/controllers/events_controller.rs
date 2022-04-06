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

/// This is a global application events bus. When an application event happens it's broadcasted by
/// this.
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
