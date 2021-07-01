use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::task::JoinHandle;
use vst::util::AtomicFloat;

use plugin_host_lib::TestPluginHost;

pub type ReceiverId = String;

pub trait IVolumeProvider: Send + 'static {
  fn volume_provider_get(&self) -> (f32, f32);
}

impl IVolumeProvider for TestPluginHost {
  fn volume_provider_get(&self) -> (f32, f32) {
    self.current_volume()
  }
}

pub struct VolumePublisherService<VolumeProvider>
where
  VolumeProvider: IVolumeProvider,
{
  host: Arc<Mutex<VolumeProvider>>,
  state: Arc<Mutex<VolumePublisherState>>,
  polling_task: Option<tokio::task::JoinHandle<()>>,
  poll_interval: Duration,
}

impl<VolumeProvider> VolumePublisherService<VolumeProvider>
where
  VolumeProvider: IVolumeProvider,
{
  pub fn new(host: Arc<Mutex<VolumeProvider>>, poll_interval: Duration) -> Self {
    VolumePublisherService {
      host,
      poll_interval: poll_interval.clone(),
      state: Arc::new(Mutex::new(VolumePublisherState::new(poll_interval))),
      polling_task: None,
    }
  }
}

impl<VolumeProvider> VolumePublisherService<VolumeProvider>
where
  VolumeProvider: IVolumeProvider,
{
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
    log::info!("Cleaning-up subscription {}", receiver_id);
    state.unsubscribe(receiver_id);
  }

  pub fn start(&mut self) {
    log::info!("Starting volume publisher poller");
    let state = self.state.clone();
    let host = self.host.clone();
    let poll_interval = self.poll_interval;
    self.polling_task = Some(tokio::spawn(async move {
      loop {
        let volume = {
          let host = host.lock().unwrap();
          host.volume_provider_get()
        };

        {
          let mut state = state.lock().unwrap();
          state.set_volume(volume);
        }

        tokio::time::sleep(poll_interval).await;
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
  poll_interval: Duration,
}

impl Default for VolumePublisherState {
  fn default() -> Self {
    Self::new(Duration::from_millis(100))
  }
}

impl VolumePublisherState {
  fn new(poll_interval: Duration) -> Self {
    VolumePublisherState {
      poll_interval,
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
    let poll_interval = self.poll_interval;
    let publisher = tokio::spawn(async move {
      loop {
        {
          let vol = (volume_left.get(), volume_right.get());
          receiver.volume_recv(vol);
        }
        tokio::time::sleep(poll_interval).await;
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
  use super::*;

  struct MockVolumeProvider {
    volume: (f32, f32),
  }

  impl IVolumeProvider for MockVolumeProvider {
    fn volume_provider_get(&self) -> (f32, f32) {
      self.volume
    }
  }

  struct TestVolume {
    volume: (f32, f32),
    call_count: usize,
  }

  #[tokio::test]
  async fn test_subscribing_to_volume() {
    let provider = Arc::new(Mutex::new(MockVolumeProvider { volume: (0.3, 0.3) }));
    let mut volume_publisher =
      VolumePublisherService::new(provider.clone(), Duration::from_millis(100));

    let test_volume_ref = Arc::new(Mutex::new(TestVolume {
      volume: (0.0, 0.0),
      call_count: 0,
    }));
    let subscription_id = volume_publisher.subscribe({
      let test_volume_ref_inner = test_volume_ref.clone();
      move |volume| {
        let mut test_volume = test_volume_ref_inner.lock().unwrap();
        (*test_volume).volume = volume;
        (*test_volume).call_count += 1;
      }
    });

    tokio::time::sleep(Duration::from_millis(130)).await;
    let test_volume = test_volume_ref.lock().unwrap();
    assert_eq!(test_volume.volume, (0.3, 0.3));
    assert_eq!(test_volume.call_count, 2);

    volume_publisher.unsubscribe(&subscription_id);
    tokio::time::sleep(Duration::from_millis(130)).await;

    assert_eq!(test_volume.volume, (0.3, 0.3));
    assert_eq!(test_volume.call_count, 2);
  }

  #[tokio::test]
  async fn test_2_subscriptions_with_50ms_publish_interval() {
    let num_subscriptions = 2;
    let provider = Arc::new(Mutex::new(MockVolumeProvider { volume: (0.3, 0.3) }));
    let mut volume_publisher =
      VolumePublisherService::new(provider.clone(), Duration::from_millis(50));

    let mut make_mock_subscription = || {
      let test_volume_ref = Arc::new(Mutex::new(TestVolume {
        volume: (0.0, 0.0),
        call_count: 0,
      }));
      volume_publisher.subscribe({
        let test_volume_ref_inner = test_volume_ref.clone();
        move |volume| {
          let mut test_volume = test_volume_ref_inner.lock().unwrap();
          (*test_volume).volume = volume;
          (*test_volume).call_count += 1;
        }
      })
    };

    let mut subscriptions = Vec::new();
    for _ in 0..num_subscriptions {
      subscriptions.push(make_mock_subscription());
    }

    tokio::time::sleep(Duration::from_millis(500)).await;
    for subscription in &subscriptions {
      volume_publisher.unsubscribe(subscription);
    }
  }
}
