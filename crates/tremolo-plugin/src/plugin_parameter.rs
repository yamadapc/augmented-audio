use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use vst::plugin::PluginParameters;

/// Trait for a single parameter
pub trait PluginParameter: Sync + Send {
    fn name(&self) -> String;
    fn label(&self) -> String;
    fn text(&self) -> String;
    fn value(&self) -> f32;
    fn set_value(&self, value: f32);
    fn can_be_automated(&self) -> bool;
}

/// Simple implementation of a parameter
pub struct PluginParameterImpl {
    /// This is the only mutable field.
    ///
    /// Not all platforms will support this. This will not work properly depending on whether the
    /// CPU supports atomic f32 instructions.
    value: crossbeam::atomic::AtomicCell<f32>,
    name: String,
    label: String,
    can_be_automated: bool,
}

unsafe impl Send for PluginParameterImpl {}
unsafe impl Sync for PluginParameterImpl {}

impl PluginParameterImpl {
    pub fn new_with(name: &str, label: &str, value: f32, can_be_automated: bool) -> Self {
        PluginParameterImpl {
            value: crossbeam::atomic::AtomicCell::new(value),
            name: String::from(name),
            label: String::from(label),
            can_be_automated,
        }
    }

    pub fn new(name: &str, label: &str) -> Self {
        PluginParameterImpl::new_with(name, label, 0.0, true)
    }
}

impl PluginParameter for PluginParameterImpl {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn text(&self) -> String {
        format!("{}", self.value.load())
    }

    fn value(&self) -> f32 {
        self.value.load()
    }

    fn set_value(&self, value: f32) {
        self.value.store(value)
    }

    fn can_be_automated(&self) -> bool {
        self.can_be_automated
    }
}

type ParameterId = String;

struct ParameterStoreInner {
    parameters: HashMap<ParameterId, Arc<dyn PluginParameter>>,
    parameter_ids: Vec<ParameterId>,
}

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

    pub fn add_parameter(&mut self, id: &str, parameter: Arc<dyn PluginParameter>) {
        if let Ok(mut inner) = self.inner.write() {
            inner.parameter_ids.push(id.to_string());
            inner.parameters.insert(id.to_string(), parameter);
        }
    }

    pub fn find_parameter(&self, parameter_id: &str) -> Option<Arc<dyn PluginParameter>> {
        let inner = self.inner.read().ok()?;
        Some(inner.parameters.get(parameter_id)?.clone())
    }

    // TODO - fix this; don't copy the ID string just to return it.
    pub fn find_parameter_by_index(
        &self,
        index: i32,
    ) -> Option<(String, Arc<dyn PluginParameter>)> {
        let inner = self.inner.read().ok()?;
        let parameter_id = inner.parameter_ids.get(index as usize)?;
        Some((
            parameter_id.clone(),
            inner.parameters.get(parameter_id)?.clone(),
        ))
    }

    pub fn get_num_parameters(&self) -> i32 {
        let run = || -> Option<i32> {
            let inner = self.inner.read().ok()?;
            Some(inner.parameter_ids.len() as i32)
        };
        run().unwrap_or(0)
    }

    pub fn value(&self, id: &str) -> f32 {
        self.find_parameter(id).as_ref().unwrap().value()
    }
}

impl PluginParameters for ParameterStore {
    fn get_parameter_label(&self, index: i32) -> String {
        let run = move || -> Option<String> {
            let (_, parameter) = self.find_parameter_by_index(index)?;
            Some(parameter.label())
        };
        run().unwrap_or_else(|| "Unknown".to_string())
    }

    fn get_parameter_text(&self, index: i32) -> String {
        let run = move || -> Option<String> {
            let (_, parameter) = self.find_parameter_by_index(index)?;
            Some(parameter.text())
        };
        run().unwrap_or_else(|| "Unknown".to_string())
    }

    fn get_parameter_name(&self, index: i32) -> String {
        let run = move || -> Option<String> {
            let (_, parameter) = self.find_parameter_by_index(index)?;
            Some(parameter.name())
        };
        run().unwrap_or_else(|| "Unknown".to_string())
    }

    fn get_parameter(&self, index: i32) -> f32 {
        let run = move || -> Option<f32> {
            let (_, parameter) = self.find_parameter_by_index(index)?;
            Some(parameter.value())
        };
        run().unwrap_or(0.0)
    }

    fn set_parameter(&self, index: i32, value: f32) {
        let run = move || -> Option<()> {
            let (_, parameter) = self.find_parameter_by_index(index)?;
            parameter.set_value(value);
            Some(())
        };
        run();
    }

    fn can_be_automated(&self, index: i32) -> bool {
        let run = move || -> Option<bool> {
            let (_, parameter) = self.find_parameter_by_index(index)?;
            Some(parameter.can_be_automated())
        };
        run().unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_creating_and_adding_parameters() {
        let mut parameter_store = ParameterStore::new();
        let parameter = Arc::new(PluginParameterImpl::new("Test parameter", "label"));
        parameter_store.add_parameter("test", parameter);

        let first_parameter_name = parameter_store.get_parameter_name(0);
        assert_eq!(first_parameter_name, "Test parameter");
    }

    #[test]
    fn test_parameter_fields() {
        let mut parameter_store = ParameterStore::new();
        let parameter = Arc::new(PluginParameterImpl::new_with(
            "Test parameter",
            "label",
            10.0,
            true,
        ));
        parameter_store.add_parameter("test", parameter);

        assert_eq!(parameter_store.get_parameter_name(0), "Test parameter");
        assert_eq!(parameter_store.get_parameter_label(0), "label");
        assert_eq!(parameter_store.get_parameter_text(0), "10");
        assert_eq!(parameter_store.get_parameter(0), 10.0);
    }

    #[test]
    fn test_parameter_set_and_get() {
        let mut parameter_store = ParameterStore::new();
        let parameter = Arc::new(PluginParameterImpl::new_with(
            "Test parameter",
            "label",
            10.0,
            true,
        ));
        parameter_store.add_parameter("test", parameter);

        let parameter = parameter_store.find_parameter("test");
        let parameter = parameter.expect("Parameter is missing");
        assert_eq!(parameter.value(), 10.0);
        parameter.set_value(20.0);
        assert_eq!(parameter.value(), 20.0);
    }

    #[test]
    fn test_float_is_atomic() {
        assert!(crossbeam::atomic::AtomicCell::<f32>::is_lock_free());
    }
}
