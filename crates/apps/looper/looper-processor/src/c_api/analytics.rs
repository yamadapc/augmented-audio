use actix_system_threads::ActorSystemThread;

use crate::services::analytics::{GetAnalyticsEnabled, SetAnalyticsEnabled};
use crate::LooperEngine;

#[no_mangle]
pub unsafe extern "C" fn looper_engine__has_analytics_enabled(engine: *mut LooperEngine) -> bool {
    let engine = &*engine;
    let analytics_service = engine.analytics_service().clone();
    ActorSystemThread::current()
        .spawn_result(async move { analytics_service.send(GetAnalyticsEnabled).await.unwrap() })
        .is_some()
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_analytics_enabled(engine: *mut LooperEngine) -> bool {
    let engine = &*engine;
    let analytics_service = engine.analytics_service().clone();
    ActorSystemThread::current()
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
    ActorSystemThread::current().spawn(async move {
        analytics_service
            .send(SetAnalyticsEnabled(enabled))
            .await
            .unwrap();
    });
}
