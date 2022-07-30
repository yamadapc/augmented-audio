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
use actix_system_threads::ActorSystem;

use crate::c_api::foreign_callback::ForeignCallback;
use crate::controllers::events_controller::{AddConsumerMessage, ApplicationEvent};
use crate::engine::LooperEngine;

#[no_mangle]
pub unsafe extern "C" fn looper_engine__register_events_callback(
    engine: *const LooperEngine,
    callback: ForeignCallback<ApplicationEvent>,
) {
    let engine = &*engine;
    let events_controller = engine.events_controller();
    let result = ActorSystem::current().spawn_result(async move {
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

    use actix_system_threads::ActorSystem;

    use crate::controllers::events_controller::{ApplicationEvent, BroadcastMessage};
    use crate::{looper_engine__register_events_callback, ForeignCallback, LooperEngine};

    extern "C" fn closure_forwarder(context: *mut c_void, value: ApplicationEvent) {
        let context: &mut &mut dyn Fn(ApplicationEvent) = unsafe { std::mem::transmute(context) };
        context(value);
    }

    #[test]
    pub fn test_looper_engine_register_events_callback() {
        let (tx, rx) = channel();
        let closure = move |value| tx.send(value).unwrap();
        let context: Box<Box<dyn Fn(ApplicationEvent)>> = Box::new(Box::new(closure));
        let context = Box::into_raw(context) as *mut c_void;
        let foreign_callback = ForeignCallback {
            context,
            callback: closure_forwarder,
        };

        let looper_engine = LooperEngine::default();
        let looper_engine = Box::into_raw(Box::new(looper_engine));

        unsafe {
            looper_engine__register_events_callback(looper_engine, foreign_callback);
        }

        let events_controller = unsafe { (*looper_engine).events_controller() };
        ActorSystem::current().spawn(async move {
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

        log::info!("Test is done, dropping values");
    }
}
