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
