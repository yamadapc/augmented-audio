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
use std::ffi::CString;
use std::os::raw::c_char;

use crate::services::analytics::{
    GetAnalyticsEnabled, SendAnalyticsEvent, ServiceAnalyticsEvent, SetAnalyticsEnabled,
};
use crate::LooperEngine;

#[repr(C)]
pub enum CAnalyticsEvent {
    ScreenView {
        content: *mut c_char,
    },
    Event {
        category: *mut c_char,
        action: *mut c_char,
        label: *mut c_char,
        value: *mut c_char,
    },
}

unsafe impl Send for CAnalyticsEvent {}

impl From<CAnalyticsEvent> for ServiceAnalyticsEvent {
    fn from(event: CAnalyticsEvent) -> ServiceAnalyticsEvent {
        unsafe {
            match event {
                CAnalyticsEvent::ScreenView { content } => ServiceAnalyticsEvent::Screen {
                    content: CString::from_raw(content)
                        .into_string()
                        .unwrap_or_else(|_| "".to_string()),
                },
                CAnalyticsEvent::Event {
                    category,
                    action,
                    label,
                    value,
                } => ServiceAnalyticsEvent::Event {
                    category: CString::from_raw(category)
                        .into_string()
                        .unwrap_or_else(|_| "".to_string()),
                    action: CString::from_raw(action)
                        .into_string()
                        .unwrap_or_else(|_| "".to_string()),
                    label: CString::from_raw(label)
                        .into_string()
                        .unwrap_or_else(|_| "".to_string()),
                    value: CString::from_raw(value)
                        .into_string()
                        .unwrap_or_else(|_| "".to_string()),
                },
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__send_analytics(
    engine: *mut LooperEngine,
    event: CAnalyticsEvent,
) {
    let engine = &*engine;
    let analytics_service = engine.analytics_service().clone();
    ActorSystem::current().spawn(async move {
        analytics_service
            .send(SendAnalyticsEvent(event.into()))
            .await
            .unwrap();
    });
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__has_analytics_enabled(engine: *mut LooperEngine) -> bool {
    let engine = &*engine;
    let analytics_service = engine.analytics_service().clone();
    ActorSystem::current()
        .spawn_result(async move { analytics_service.send(GetAnalyticsEnabled).await.unwrap() })
        .is_some()
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_analytics_enabled(engine: *mut LooperEngine) -> bool {
    let engine = &*engine;
    let analytics_service = engine.analytics_service().clone();
    ActorSystem::current()
        .spawn_result(async move { analytics_service.send(GetAnalyticsEnabled).await.unwrap() })
        .unwrap_or(false)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_analytics_enabled(
    engine: *mut LooperEngine,
    enabled: bool,
) {
    let engine = &*engine;
    let analytics_service = engine.analytics_service().clone();
    ActorSystem::current().spawn(async move {
        analytics_service
            .send(SetAnalyticsEnabled(enabled))
            .await
            .unwrap();
    });
}
