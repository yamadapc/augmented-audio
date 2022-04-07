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

use std::ffi::CString;
use std::os::raw::c_char;

use crate::c_api::into_ptr;
use crate::services::effects_service::{EffectDefinition, EffectParameterModel, EffectsService};
use crate::LooperEngine;

#[repr(C)]
pub struct CEffectDefinitionsList {
    inner: *mut Vec<*mut EffectDefinition>,
}

#[no_mangle]
pub unsafe extern "C" fn effect_definition__name(definition: *mut EffectDefinition) -> *mut c_char {
    let definition = &(*definition);
    CString::new(definition.name.clone())
        .unwrap_or_else(|_| CString::new("unknown").unwrap())
        .into_raw()
}

#[repr(C)]
pub struct CEffectParameterList {
    inner: *mut Vec<*mut EffectParameterModel>,
}

#[no_mangle]
pub unsafe extern "C" fn effect_parameters__count(parameters: *mut CEffectParameterList) -> usize {
    (*(*parameters).inner).len()
}

#[no_mangle]
pub unsafe extern "C" fn effect_parameters__get(
    parameters: *mut CEffectParameterList,
    index: usize,
) -> *mut EffectParameterModel {
    (*(*parameters).inner)[index]
}

#[no_mangle]
pub unsafe extern "C" fn effect_parameter__label(
    parameter: *mut EffectParameterModel,
) -> *mut c_char {
    let parameter = &(*parameter);
    CString::new(parameter.spec.name())
        .unwrap_or_else(|_| CString::new("unknown").unwrap())
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn effect_definition__parameters(
    definition: *mut EffectDefinition,
) -> *mut CEffectParameterList {
    let definition = &(*definition);
    into_ptr(CEffectParameterList {
        inner: into_ptr(
            definition
                .parameters
                .clone()
                .into_iter()
                .map(into_ptr)
                .collect(),
        ),
    })
}

#[no_mangle]
pub unsafe extern "C" fn effect_definitions__get(
    list: *mut CEffectDefinitionsList,
    index: usize,
) -> *mut EffectDefinition {
    (*(*list).inner)[index]
}

#[no_mangle]
pub unsafe extern "C" fn effect_definitions__count(list: *mut CEffectDefinitionsList) -> usize {
    (*(*list).inner).len()
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_effect_definitions() -> *mut CEffectDefinitionsList {
    let definitions = EffectsService::get_effects();
    let definitions = definitions.into_iter().map(into_ptr).collect();
    into_ptr(CEffectDefinitionsList {
        inner: into_ptr(definitions),
    })
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_effect(
    engine: *mut LooperEngine,
    looper_id: usize,
    effect_type: usize,
) {
    let handle = (*engine).handle();
    let definitions = EffectsService::get_effects();
    let effect_type = definitions[effect_type].ty.clone();

    handle.voices()[looper_id].effects().add_effect(effect_type);
}
