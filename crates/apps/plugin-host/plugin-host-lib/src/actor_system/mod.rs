use lazy_static::lazy_static;
use std::future::Future;
use std::sync::mpsc::channel;

use actix::prelude::*;

lazy_static! {
    static ref THREAD: ActorSystemThread = ActorSystemThread::with_new_system();
}

#[derive(Debug, Clone)]
pub struct ActorSystemThread {
    arbiter_handle: ArbiterHandle,
}

impl Default for ActorSystemThread {
    fn default() -> Self {
        THREAD.clone()
    }
}

impl ActorSystemThread {
    pub fn current() -> Self {
        THREAD.clone()
    }

    fn with_new_system() -> Self {
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let system = actix::System::new();
            let arbiter_handle = Arbiter::current();
            let _ = tx.send(arbiter_handle);
            system.run()
        });
        let arbiter_handle = rx.recv().unwrap();

        Self { arbiter_handle }
    }

    #[allow(unused)]
    pub fn spawn<Fut>(&self, fut: Fut)
    where
        Fut: 'static + Send + Future<Output = ()>,
    {
        self.arbiter_handle.spawn(fut);
    }

    #[allow(unused)]
    pub fn spawn_result<Fut, T>(&self, fut: Fut) -> T
    where
        Fut: 'static + Send + Future<Output = T>,
        T: 'static + Send,
    {
        let (tx, rx) = channel();
        self.arbiter_handle.spawn(async move {
            let result = fut.await;
            let _ = tx.send(result);
        });
        let result = rx.recv().unwrap();
        result
    }
}

impl Drop for ActorSystemThread {
    fn drop(&mut self) {
        log::info!("Stopping actor system thread");
        self.arbiter_handle.spawn(async {
            System::current().stop();
        });
        let _ = self.arbiter_handle.stop();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_start_actor_system_thread() {
        let _ = wisual_logger::try_init_from_env();
        let _actor_system_thread = ActorSystemThread::default();
    }

    #[test]
    fn test_spawn_actor() {
        struct TestActor {}
        impl Actor for TestActor {
            type Context = Context<TestActor>;
        }

        #[derive(Message)]
        #[rtype(result = "String")]
        struct Ping;

        impl Handler<Ping> for TestActor {
            type Result = String;

            fn handle(&mut self, _msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
                "pong".to_string()
            }
        }

        let _ = wisual_logger::try_init_from_env();
        let actor_system_thread = ActorSystemThread::default();

        let addr: Addr<TestActor> =
            actor_system_thread.spawn_result(async { TestActor {}.start() });

        let result = actor_system_thread
            .spawn_result(async move { addr.send(Ping).await })
            .unwrap();
        assert_eq!(result, "pong");
    }
}
