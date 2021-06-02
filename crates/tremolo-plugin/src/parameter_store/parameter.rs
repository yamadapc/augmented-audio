/// Simple implementation of a parameter
pub struct PluginParameter {
    /// This is the only mutable field.
    ///
    /// Not all platforms will support this. This will not work properly depending on whether the
    /// CPU supports atomic f32 instructions.
    value: crossbeam::atomic::AtomicCell<f32>,
    name: String,
    label: String,
    can_be_automated: bool,
}

unsafe impl Send for PluginParameter {}
unsafe impl Sync for PluginParameter {}

impl PluginParameter {
    pub fn new_with(name: &str, label: &str, value: f32, can_be_automated: bool) -> Self {
        PluginParameter {
            value: crossbeam::atomic::AtomicCell::new(value),
            name: String::from(name),
            label: String::from(label),
            can_be_automated,
        }
    }

    pub fn new(name: &str, label: &str) -> Self {
        PluginParameter::new_with(name, label, 0.0, true)
    }
}

impl PluginParameter {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn text(&self) -> String {
        format!("{}", self.value.load())
    }

    pub fn value(&self) -> f32 {
        self.value.load()
    }

    pub fn set_value(&self, value: f32) {
        self.value.store(value)
    }

    pub fn can_be_automated(&self) -> bool {
        self.can_be_automated
    }
}
