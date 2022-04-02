use crate::audio::multi_track_looper::effects_processor::EffectType;
use crate::LooperEngine;

#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_effect(
    engine: *mut LooperEngine,
    looper_id: usize,
    effect_type: EffectType,
) {
    let handle = (*engine).handle();
    handle.voices()[looper_id].effects().add_effect(effect_type);
}
