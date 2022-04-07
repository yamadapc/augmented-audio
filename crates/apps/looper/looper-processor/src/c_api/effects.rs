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

#[no_mangle]
pub unsafe extern "C" fn c_string_free(str: *mut c_char) {
    let _ = CString::from_raw(str);
}

#[repr(C)]
pub struct CEffectDefinitionsList {
    inner: *mut Vec<*mut EffectDefinition>,
}

impl Drop for CEffectDefinitionsList {
    fn drop(&mut self) {
        unsafe {
            let v = Box::from_raw(self.inner);
            for el in v.iter() {
                let _ = Box::from_raw(*el);
            }
        }
    }
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

/// Free this parameters list and all its children
#[no_mangle]
pub unsafe extern "C" fn effect_parameters__free(parameters: *mut CEffectParameterList) {
    let _ = Box::from_raw(parameters);
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

/// List parameters in a definition. This will return a heap allocated list with heap allocated
/// `EffectParameterModel`. The list owns its children.
///
/// If the parent `EffectDefinition` is free-ed while this is used this have dangling pointers.
///
/// The caller must call `effect_parameters__free` after using this list.
///
/// Rust example:
/// ```
/// use std::ffi::CStr;
/// use looper_processor::c_api::effects::*;
///
/// // for clarity, all allocating functions are indented into a block
/// unsafe {
///     let list = looper_engine__get_effect_definitions();
///     let effect1 = effect_definitions__get(list, 0);
///
///     {
///         let effect1_name = effect_definition__name(effect1);
///         assert_eq!(CStr::from_ptr(effect1_name).to_str().unwrap(), "Reverb");
///         c_string_free(effect1_name);
///     }
///
///     {
///         let parameters_list = effect_definition__parameters(effect1);
///         let parameter1 = effect_parameters__get(parameters_list, 0);
///         {
///             let parameter1_name = effect_parameter__label(parameter1);
///             assert_eq!(CStr::from_ptr(parameter1_name).to_str().unwrap(), "Dry");
///             c_string_free(parameter1_name);
///         }
///         effect_parameters__free(parameters_list);
///     }
///
///     effect_definitions__free(list);
/// }
/// ```
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

/// Get definition at this position
#[no_mangle]
pub unsafe extern "C" fn effect_definitions__get(
    list: *mut CEffectDefinitionsList,
    index: usize,
) -> *mut EffectDefinition {
    (*(*list).inner)[index]
}

/// Get the count of effect definitions in this list
#[no_mangle]
pub unsafe extern "C" fn effect_definitions__count(list: *mut CEffectDefinitionsList) -> usize {
    (*(*list).inner).len()
}

/// Free the definitions list and its children.
#[no_mangle]
pub unsafe extern "C" fn effect_definitions__free(list: *mut CEffectDefinitionsList) {
    let _ = Box::from_raw(list);
}

/// List all available effects and their parameters, caller must call `effect_definitions__free`
/// when done.
///
/// This returns a heap allocated list of heap allocated effect definition structs. The list owns
/// the effect definition pointers it references and freeing the list will free them.
///
/// Rust example:
/// ```
/// use looper_processor::c_api::effects::*;
///
/// unsafe {
///     let list = looper_engine__get_effect_definitions();
///     effect_definitions__free(list);
/// }
/// ```
#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_effect_definitions() -> *mut CEffectDefinitionsList {
    let definitions = EffectsService::get_effects();
    let definitions = definitions.into_iter().map(into_ptr).collect();
    into_ptr(CEffectDefinitionsList {
        inner: into_ptr(definitions),
    })
}

/// Add an effect of type `EffectType` into the track with `LooperId`.
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
