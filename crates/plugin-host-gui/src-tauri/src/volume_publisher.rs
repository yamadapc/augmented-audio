use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::task::JoinHandle;
use vst::util::AtomicFloat;

use plugin_host_lib::TestPluginHost;

pub type ReceiverId = String;

pub struct VolumePublisherService {
  host: Arc<Mutex<TestPluginHost>>,
  state: Arc<Mutex<VolumePublisherState>>,
  polling_task: Option<tokio::task::JoinHandle<()>>,
}

impl VolumePublisherService {
  pub fn new(host: Arc<Mutex<TestPluginHost>>) -> Self {
    VolumePublisherService {
      host,
      state: Arc::new(Mutex::new(VolumePublisherState::new())),
      polling_task: None,
    }
  }
}

impl VolumePublisherService {
  pub fn subscribe<Cb>(&mut self, callback: Cb) -> ReceiverId
  where
    Cb: FnMut((f32, f32)) + Send + 'static,
  {
    if self.polling_task.is_none() {
      self.start();
    }

    let callback = callback;
    let receiver_id = uuid::Uuid::new_v4().to_string();
    let mut state = self.state.lock().unwrap();
    state.subscribe(receiver_id.clone(), callback);
    log::info!("Created subscription {}", receiver_id);
    receiver_id
  }

  pub fn unsubscribe(&mut self, receiver_id: &str) {
    let mut state = self.state.lock().unwrap();
    state.unsubscribe(receiver_id);
  }

  pub fn start(&mut self) {
    log::info!("Starting volume publisher poller");
    let state = self.state.clone();
    let host = self.host.clone();
    self.polling_task = Some(tokio::spawn(async move {
      loop {
        let volume = {
          let host = host.lock().unwrap();
          host.current_volume()
        };

        {
          let mut state = state.lock().unwrap();
          state.set_volume(volume);
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
      }
    }));
  }

  pub fn stop(&mut self) {
    log::info!("Stopping volume publisher loops");
    if let Some(task) = self.polling_task.take() {
      task.abort();
    }
    let mut state = self.state.lock().unwrap();
    state.unsubscribe_all();
  }
}

pub trait VolumeReceiver {
  fn volume_recv(&mut self, volume: (f32, f32));
}

impl<Cb> VolumeReceiver for Cb
where
  Cb: FnMut((f32, f32)),
{
  fn volume_recv(&mut self, volume: (f32, f32)) {
    self(volume)
  }
}

struct VolumePublisherState {
  publishers: HashMap<ReceiverId, JoinHandle<()>>,
  volume_left: Arc<AtomicFloat>,
  volume_right: Arc<AtomicFloat>,
}

impl VolumePublisherState {
  fn new() -> Self {
    VolumePublisherState {
      volume_left: Arc::new(AtomicFloat::new(0.0)),
      volume_right: Arc::new(AtomicFloat::new(0.0)),
      publishers: HashMap::new(),
    }
  }

  fn set_volume(&mut self, volume: (f32, f32)) {
    self.volume_left.set(volume.0);
    self.volume_right.set(volume.1);
  }

  fn subscribe<V: 'static + VolumeReceiver + Send>(
    &mut self,
    receiver_id: String,
    mut receiver: V,
  ) {
    let volume_left = self.volume_left.clone();
    let volume_right = self.volume_right.clone();
    let publisher = tokio::spawn(async move {
      loop {
        {
          let vol = (volume_left.get(), volume_right.get());
          receiver.volume_recv(vol);
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
      }
    });
    self.publishers.insert(receiver_id, publisher);
  }

  fn unsubscribe_all(&mut self) {
    for publisher in self.publishers.values() {
      publisher.abort();
    }
  }

  fn unsubscribe(&mut self, receiver_id: &str) {
    if let Some(publisher) = self.publishers.remove(receiver_id) {
      publisher.abort();
    }
  }
}

impl Drop for VolumePublisherState {
  fn drop(&mut self) {
    self.unsubscribe_all()
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test_subscribing_to_volume() {}
}
