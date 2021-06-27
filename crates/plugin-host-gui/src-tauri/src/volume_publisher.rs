use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub type ReceiverId = String;

pub trait VolumeReceiver {
  fn volume_recv(&mut self, volume: f32);
}

pub struct VolumePublisherState {
  #[allow(dead_code)]
  volume: Arc<RwLock<f32>>,
  publishers: HashMap<ReceiverId, JoinHandle<()>>,
}

impl VolumePublisherState {
  #[allow(dead_code)]
  pub fn new() -> Self {
    VolumePublisherState {
      volume: Arc::new(RwLock::new(0.0)),
      publishers: HashMap::new(),
    }
  }

  #[allow(dead_code)]
  pub async fn set_volume(&mut self, volume: f32) {
    let mut volume_lock = self.volume.write().await;
    *volume_lock = volume;
  }

  #[allow(dead_code)]
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

  #[allow(dead_code)]
  pub fn unsubscribe(&mut self, receiver_id: &str) {
    if let Some(publisher) = self.publishers.remove(receiver_id) {
      publisher.abort();
    }
  }
}

impl Drop for VolumePublisherState {
  fn drop(&mut self) {
    for publisher in self.publishers.values() {
      publisher.abort();
    }
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test_subscribing_to_volume() {}
}
