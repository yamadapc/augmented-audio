use crate::collector::GarbageCollector;
use crate::node::Node;
use crate::shared::SharedGeneric;

pub mod atomic_queue;
pub mod collector;
mod node;
pub mod shared;

pub type Shared<T> = SharedGeneric<T, GarbageCollector, Node<T>>;
