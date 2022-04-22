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

impl Into<ServiceAnalyticsEvent> for CAnalyticsEvent {
    fn into(self) -> ServiceAnalyticsEvent {
        unsafe {
            match self {
                CAnalyticsEvent::ScreenView { content } => ServiceAnalyticsEvent::Screen {
                    content: CString::from_raw(content)
                        .into_string()
                        .unwrap_or("".to_string()),
                },
                CAnalyticsEvent::Event {
                    category,
                    action,
                    label,
                    value,
                } => ServiceAnalyticsEvent::Event {
                    category: CString::from_raw(category)
                        .into_string()
                        .unwrap_or("".to_string()),
                    action: CString::from_raw(action)
                        .into_string()
                        .unwrap_or("".to_string()),
                    label: CString::from_raw(label)
                        .into_string()
                        .unwrap_or("".to_string()),
                    value: CString::from_raw(value)
                        .into_string()
                        .unwrap_or("".to_string()),
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
