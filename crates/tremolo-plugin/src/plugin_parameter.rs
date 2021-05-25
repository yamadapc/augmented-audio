use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use vst::plugin::PluginParameters;

/// Trait for a single parameter
pub trait PluginParameter {
    fn name(&self) -> String;
    fn label(&self) -> String;
    fn text(&self) -> String;
    fn value(&self) -> f32;
    fn set_value(&mut self, value: f32);
    fn can_be_automated(&self) -> bool;
}

/// Simple implementation of a parameter
pub struct PluginParameterImpl {
    name: String,
    label: String,
    value: f32,
    can_be_automated: bool,
}

impl PluginParameterImpl {
    pub fn new_with(name: String, label: String, value: f32, can_be_automated: bool) -> Self {
        PluginParameterImpl {
            name,
            label,
            value,
            can_be_automated,
        }
    }

    pub fn new(name: String, label: String) -> Self {
        PluginParameterImpl {
            name,
            label,
            value: 0.0,
            can_be_automated: true,
        }
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
        format!("{}", self.value)
    }

    fn value(&self) -> f32 {
        self.value
    }

    fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    fn can_be_automated(&self) -> bool {
        self.can_be_automated
    }
}

type ParameterId = String;

struct ParameterStoreInner {
    parameters: HashMap<ParameterId, Arc<Mutex<dyn PluginParameter + Send>>>,
    parameter_ids: Vec<ParameterId>,
}

/// Holder of parameters
pub struct ParameterStore {
    inner: Mutex<ParameterStoreInner>,
}

impl Default for ParameterStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ParameterStore {
    pub fn new() -> Self {
        ParameterStore {
            inner: Mutex::new(ParameterStoreInner {
                parameters: HashMap::new(),
                parameter_ids: Vec::new(),
            }),
        }
    }

    pub fn add_parameter(
        &mut self,
        id: ParameterId,
        parameter: Arc<Mutex<dyn PluginParameter + Send>>,
    ) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.parameter_ids.push(id.clone());
            inner.parameters.insert(id, parameter);
        }
    }

    pub fn find_parameter(&self, index: i32) -> Option<Arc<Mutex<dyn PluginParameter + Send>>> {
        let inner = self.inner.lock().ok()?;
        let parameter_id = inner.parameter_ids.get(index as usize)?;
        Some(inner.parameters.get(parameter_id)?.clone())
    }

    pub fn get_num_parameters(&self) -> i32 {
        let run = || -> Option<i32> {
            let inner = self.inner.lock().ok()?;
            Some(inner.parameter_ids.len() as i32)
        };
        run().unwrap_or(0)
    }
}

impl PluginParameters for ParameterStore {
    fn get_parameter_label(&self, index: i32) -> String {
        let run = move || -> Option<String> {
            let parameter = self.find_parameter(index)?;
            let parameter = parameter.lock().ok()?;
            Some(parameter.label())
        };
        run().unwrap_or("Unknown".to_string())
    }

    fn get_parameter_text(&self, index: i32) -> String {
        let run = move || -> Option<String> {
            let parameter = self.find_parameter(index)?;
            let parameter = parameter.lock().ok()?;
            Some(parameter.text())
        };
        run().unwrap_or("Unknown".to_string())
    }

    fn get_parameter_name(&self, index: i32) -> String {
        let run = move || -> Option<String> {
            let parameter = self.find_parameter(index)?;
            let parameter = parameter.lock().ok()?;
            Some(parameter.name())
        };
        run().unwrap_or("Unknown".to_string())
    }

    fn get_parameter(&self, index: i32) -> f32 {
        let run = move || -> Option<f32> {
            let parameter = self.find_parameter(index)?;
            let parameter = parameter.lock().ok()?;
            Some(parameter.value())
        };
        run().unwrap_or(0.0)
    }

    fn set_parameter(&self, index: i32, value: f32) {
        let run = move || -> Option<()> {
            let parameter = self.find_parameter(index)?;
            let mut parameter = parameter.lock().ok()?;
            parameter.set_value(value);
            Some(())
        };
        run();
    }

    fn can_be_automated(&self, index: i32) -> bool {
        let run = move || -> Option<bool> {
            let parameter = self.find_parameter(index)?;
            let parameter = parameter.lock().ok()?;
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
        let parameter = Arc::new(Mutex::new(PluginParameterImpl::new(
            "Test parameter".to_string(),
            "label".to_string(),
        )));
        parameter_store.add_parameter("test".to_string(), parameter);

        let first_parameter_name = parameter_store.get_parameter_name(0);
        assert_eq!(first_parameter_name, "Test parameter".to_string());
    }

    #[test]
    fn test_parameter_fields() {
        let mut parameter_store = ParameterStore::new();
        let parameter = Arc::new(Mutex::new(PluginParameterImpl::new_with(
            "Test parameter".to_string(),
            "label".to_string(),
            10.0,
            true,
        )));
        parameter_store.add_parameter("test".to_string(), parameter);

        assert_eq!(
            parameter_store.get_parameter_name(0),
            "Test parameter".to_string()
        );
        assert_eq!(parameter_store.get_parameter_label(0), "label".to_string());
        assert_eq!(parameter_store.get_parameter_text(0), "10".to_string());
        assert_eq!(parameter_store.get_parameter(0), 10.0);
    }
}
