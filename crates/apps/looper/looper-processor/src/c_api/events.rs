use actix_system_threads::ActorSystemThread;

use crate::c_api::foreign_callback::ForeignCallback;
use crate::controllers::events_controller::{AddConsumerMessage, ApplicationEvent};
use crate::engine::LooperEngine;

#[no_mangle]
pub extern "C" fn looper_engine__register_events_callback(
    engine: *mut LooperEngine,
    callback: ForeignCallback<ApplicationEvent>,
) {
    let engine = unsafe { &*engine };
    let events_controller = engine.events_controller();
    let result = ActorSystemThread::current().spawn_result(async move {
        events_controller
            .send(AddConsumerMessage(Box::new(callback)))
            .await
    });
    if let Err(err) = result {
        log::error!(
            "Failure sending add consumer message to controller: {}",
            err
        );
    }
}

#[cfg(test)]
mod test {
    use std::ffi::c_void;
    use std::sync::mpsc::channel;

    use actix_system_threads::ActorSystemThread;

    use crate::controllers::events_controller::{ApplicationEvent, BroadcastMessage};
    use crate::{looper_engine__register_events_callback, ForeignCallback, LooperEngine};

    extern "C" fn closure_forwarder(context: *mut c_void, value: ApplicationEvent) {
        let context: &mut &mut dyn Fn(ApplicationEvent) -> () =
            unsafe { std::mem::transmute(context) };
        context(value);
    }

    #[test]
    pub fn test_looper_engine_register_events_callback() {
        let (tx, rx) = channel();
        let closure = move |value| tx.send(value).unwrap();
        let context: Box<Box<dyn Fn(ApplicationEvent) -> ()>> = Box::new(Box::new(closure));
        let context = Box::into_raw(context) as *mut c_void;
        let foreign_callback = ForeignCallback {
            context,
            callback: closure_forwarder,
        };

        let looper_engine = LooperEngine::new();
        let looper_engine = Box::into_raw(Box::new(looper_engine));

        looper_engine__register_events_callback(looper_engine, foreign_callback);

        let events_controller = unsafe { (*looper_engine).events_controller() };
        ActorSystemThread::current().spawn(async move {
            events_controller
                .send(BroadcastMessage(
                    ApplicationEvent::ApplicationEventLooperClipUpdated { looper_id: 10 },
                ))
                .await
                .unwrap();
        });
        let event = rx.recv().unwrap();
        assert_eq!(
            event,
            ApplicationEvent::ApplicationEventLooperClipUpdated { looper_id: 10 }
        );
    }
}
