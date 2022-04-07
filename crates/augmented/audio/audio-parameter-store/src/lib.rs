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
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub use parameter::PluginParameter;
pub use parameter::PluginParameterLike;

pub mod parameter;

/// Holder of parameters
///
/// Uses `RwLock` and atomics internally.
///
/// Modifying parameter values or adding/removing parameters will lock the store for all threads.
/// Setting parameter values as well as reading them will not lock as long as there isn't a writer
/// adding/removing parameters.
///
/// The parameters themselves wrap an atomic value & otherwise immutable fields.
///
/// I should validate that this is sound.
pub struct ParameterStore {
    inner: RwLock<ParameterStoreInner>,
}

unsafe impl Send for ParameterStore {}
unsafe impl Sync for ParameterStore {}

impl Default for ParameterStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ParameterStore {
    pub fn new() -> Self {
        ParameterStore {
            inner: RwLock::new(ParameterStoreInner {
                parameters: HashMap::new(),
                parameter_ids: Vec::new(),
            }),
        }
    }

    /// Add a parameter to the store
    ///
    /// # Locking
    /// This will lock the store for writing.
    /// Locking the store while processing audio is not desired.
    pub fn add_parameter(&mut self, id: &str, parameter: ParameterRef) {
        if let Ok(mut inner) = self.inner.write() {
            inner.parameter_ids.push(id.to_string());
            inner.parameters.insert(id.to_string(), parameter);
        }
    }

    /// Find a parameter by ID
    ///
    /// # Locking
    /// This will block if the store is locked for writing.
    pub fn find_parameter(&self, parameter_id: &str) -> Option<ParameterRef> {
        let inner = self.inner.read().ok()?;
        Some(inner.parameters.get(parameter_id)?.clone())
    }

    /// Find a parameter ID by index
    ///
    /// # Locking
    /// This will block if the store is locked for writing.
    pub fn find_parameter_id(&self, index: i32) -> Option<String> {
        let inner = self.inner.read().ok()?;
        Some(inner.parameter_ids.get(index as usize)?.clone())
    }

    /// Find a parameter by index
    ///
    /// # Locking
    /// This will block if the store is locked for writing.
    pub fn find_parameter_by_index(&self, index: i32) -> Option<ParameterRef> {
        let inner = self.inner.read().ok()?;
        let parameter_id = inner.parameter_ids.get(index as usize)?;
        Some(inner.parameters.get(parameter_id)?.clone())
    }

    /// Get count of parameters
    ///
    /// # Locking
    /// This will block if the store is locked for writing.
    pub fn get_num_parameters(&self) -> i32 {
        let run = || -> Option<i32> {
            let inner = self.inner.read().ok()?;
            Some(inner.parameter_ids.len() as i32)
        };
        run().unwrap_or(0)
    }

    /// Get a parameter value by ID
    ///
    /// # Locking
    /// This will block if the store is locked for writing.
    pub fn value(&self, id: &str) -> f32 {
        self.find_parameter(id).as_ref().unwrap().value()
    }
}

impl vst::plugin::PluginParameters for ParameterStore {
    fn get_parameter_label(&self, index: i32) -> String {
        let run = move || -> Option<String> {
            let parameter = self.find_parameter_by_index(index)?;
            Some(parameter.label())
        };
        run().unwrap_or_else(|| "Unknown".to_string())
    }

    fn get_parameter_text(&self, index: i32) -> String {
        let run = move || -> Option<String> {
            let parameter = self.find_parameter_by_index(index)?;
            Some(parameter.text())
        };
        run().unwrap_or_else(|| "Unknown".to_string())
    }

    fn get_parameter_name(&self, index: i32) -> String {
        let run = move || -> Option<String> {
            let parameter = self.find_parameter_by_index(index)?;
            Some(parameter.name())
        };
        run().unwrap_or_else(|| "Unknown".to_string())
    }

    fn get_parameter(&self, index: i32) -> f32 {
        let run = move || -> Option<f32> {
            let parameter = self.find_parameter_by_index(index)?;
            Some(parameter.value())
        };
        run().unwrap_or(0.0)
    }

    fn set_parameter(&self, index: i32, value: f32) {
        let run = move || -> Option<()> {
            let parameter = self.find_parameter_by_index(index)?;
            parameter.set_value(value);
            Some(())
        };
        run();
    }

    fn can_be_automated(&self, index: i32) -> bool {
        let run = move || -> Option<bool> {
            let parameter = self.find_parameter_by_index(index)?;
            Some(parameter.can_be_automated())
        };
        run().unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;
    use vst::plugin::PluginParameters;

    use super::*;

    #[test]
    fn test_creating_and_adding_parameters() {
        let mut parameter_store = ParameterStore::new();
        let parameter = Arc::new(PluginParameter::builder().name("Test parameter").build());
        parameter_store.add_parameter("test", parameter);

        let first_parameter_name = parameter_store.get_parameter_name(0);
        assert_eq!(first_parameter_name, "Test parameter");
    }

    #[test]
    fn test_parameter_fields() {
        let mut parameter_store = ParameterStore::new();
        let parameter = Arc::new(
            PluginParameter::builder()
                .name("Test parameter")
                .label("label")
                .initial_value(10.0)
                .build(),
        );
        parameter_store.add_parameter("test", parameter);

        assert_eq!(parameter_store.get_parameter_name(0), "Test parameter");
        assert_eq!(parameter_store.get_parameter_label(0), "label");
        assert_eq!(parameter_store.get_parameter_text(0), "10");
        assert_f_eq!(parameter_store.get_parameter(0), 10.0);
    }

    #[test]
    fn test_parameter_set_and_get() {
        let mut parameter_store = ParameterStore::new();
        let parameter = Arc::new(
            PluginParameter::builder()
                .name("Test parameter")
                .initial_value(10.0)
                .build(),
        );
        parameter_store.add_parameter("test", parameter);

        let parameter = parameter_store.find_parameter("test");
        let parameter = parameter.expect("Parameter is missing");
        assert_f_eq!(parameter.value(), 10.0);
        parameter.set_value(20.0);
        assert_f_eq!(parameter.value(), 20.0);
    }

    #[test]
    fn test_float_is_atomic() {
        assert!(crossbeam::atomic::AtomicCell::<f32>::is_lock_free());
    }
}

/// Parameter IDs are strings
pub type ParameterId = String;
pub type ParameterRef = Arc<dyn PluginParameterLike>;

struct ParameterStoreInner {
    parameters: HashMap<ParameterId, ParameterRef>,
    parameter_ids: Vec<ParameterId>,
}
