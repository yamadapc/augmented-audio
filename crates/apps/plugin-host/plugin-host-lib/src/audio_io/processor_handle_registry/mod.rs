use std::any::Any;
use std::sync::Arc;

use concread::hashmap::HashMap;
use lazy_static::lazy_static;

use audio_garbage_collector::Shared;

lazy_static! {
    static ref PROCESSOR_HANDLE_REGISTRY: ProcessorHandleRegistry =
        ProcessorHandleRegistry::default();
}

pub type HandleId = String;

/// Shared storage of 'processor handles'.
#[derive(Default)]
pub struct ProcessorHandleRegistry {
    handles: HashMap<HandleId, Arc<dyn Any + 'static + Sync + Send>>,
}

impl ProcessorHandleRegistry {
    pub fn register(
        &self,
        id: impl Into<HandleId>,
        handle: impl Any + 'static + Sized + Sync + Send,
    ) {
        let mut tx = self.handles.write();
        tx.insert(id.into(), Arc::new(handle));
        tx.commit();
    }

    pub fn get<T: 'static>(&self, id: &str) -> Option<Shared<T>> {
        let tx = self.handles.read();
        let entry = tx.get(id)?;
        let ptr = entry.downcast_ref::<Shared<T>>()?;
        Some(ptr.clone())
    }

    pub fn current() -> &'static Self {
        &PROCESSOR_HANDLE_REGISTRY
    }
}

#[cfg(test)]
mod test {
    use audio_garbage_collector::GarbageCollector;
    use audio_processor_traits::AtomicF32;

    use super::*;

    #[test]
    fn test_register_handle() {
        #[derive(Default)]
        struct Handle {
            #[allow(unused)]
            volume: AtomicF32,
        }

        let gc = GarbageCollector::default();
        let registry = ProcessorHandleRegistry::default();
        let handle = Shared::new(gc.handle(), Handle::default());
        registry.register("handle", handle);
        let result: Option<Shared<Handle>> = registry.get("handle");

        assert!(result.is_some());
    }
}
