use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub type ReceiverId = String;

pub trait VolumeReceiver {
  fn volume_recv(self: &mut Self, volume: f32);
}

pub struct VolumePublisherState {
  volume: Arc<RwLock<f32>>,
  publishers: HashMap<ReceiverId, JoinHandle<()>>,
}

impl VolumePublisherState {
  pub fn new() -> Self {
    VolumePublisherState {
      volume: Arc::new(RwLock::new(0.0)),
      publishers: HashMap::new(),
    }
  }

  pub fn subscribe<V: 'static + VolumeReceiver + Send>(
    &mut self,
    receiver_id: String,
    mut receiver: V,
  ) {
    let volume = self.volume.clone();
    let publisher = tokio::spawn(async move {
      loop {
        let vol = volume.read().await;
        receiver.volume_recv(*vol);
        tokio::time::sleep(Duration::from_millis(100)).await;
      }
    });
    self.publishers.insert(receiver_id, publisher);
  }

  pub fn unsubscribe(&mut self, receiver_id: &str) {
    if let Some(publisher) = self.publishers.remove(receiver_id) {
      publisher.abort();
    }
  }
}

impl Drop for VolumePublisherState {
  fn drop(&mut self) {
    for (_id, publisher) in &self.publishers {
      publisher.abort();
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_subscribing_to_volume() {}
}
