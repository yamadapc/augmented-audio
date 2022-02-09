use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;

use actix::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref THREAD: ActorSystemThread = {
        log::info!("Global actor system will start");
        ActorSystemThread::with_new_system()
    };
}

#[derive(Debug)]
pub struct ActorSystemThread {
    #[allow(dead_code)]
    system: actix::System,
    arbiters: Vec<ArbiterHandle>,
    counter: Arc<AtomicUsize>,
}

impl ActorSystemThread {
    pub fn current() -> &'static Self {
        &THREAD
    }

    fn with_new_system() -> Self {
        let (tx, rx) = channel();
        let (sys_tx, sys_rx) = channel();

        log::info!("Starting actor system on 8 threads");
        std::thread::Builder::new()
            .name("actor-system-main".into())
            .spawn(move || {
                let system = actix::System::new();
                let mut arbiters = vec![Arbiter::current()];
                for _ in 0..8 {
                    arbiters.push(Arbiter::new().handle());
                }
                tx.send(arbiters).unwrap();
                sys_tx.send(System::current()).unwrap();
                system.run().unwrap();
                log::warn!("System has stopped");
            })
            .unwrap();

        let arbiters = rx.recv().unwrap();
        let system = sys_rx.recv().unwrap();

        Self {
            system,
            arbiters,
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    #[allow(unused)]
    pub fn spawn<Fut>(&self, fut: Fut)
    where
        Fut: 'static + Send + Future<Output = ()>,
    {
        let target_id = self.next_arbiter_idx();
        log::debug!("Spawning task on arbiter_id={}", target_id);
        self.arbiters[target_id].spawn(fut);
    }

    #[allow(unused)]
    pub fn spawn_result<Fut, T>(&self, fut: Fut) -> T
    where
        Fut: 'static + Send + Future<Output = T>,
        T: 'static + Send,
    {
        let (tx, rx) = channel();
        self.spawn(async move {
            let result = fut.await;
            let _ = tx.send(result);
        });

        rx.recv().unwrap()
    }

    fn next_arbiter_idx(&self) -> usize {
        self.counter.fetch_add(1, Ordering::Relaxed) % self.arbiters.len()
    }

    pub fn start<A>(actor: A) -> Addr<A>
    where
        A: Actor<Context = actix::Context<A>> + Send,
    {
        Self::current().spawn_result(async move { actor.start() })
    }
}

impl Drop for ActorSystemThread {
    fn drop(&mut self) {
        log::info!("Stopping actor system thread");
        self.spawn(async {
            System::current().stop();
        });
        for arbiter in &self.arbiters {
            let _ = arbiter.stop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_start_actor_system_thread() {
        let _ = wisual_logger::try_init_from_env();
        let _actor_system_thread = ActorSystemThread::with_new_system();
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
        let actor_system_thread = ActorSystemThread::with_new_system();

        let addr: Addr<TestActor> =
            actor_system_thread.spawn_result(async { TestActor {}.start() });

        let result = actor_system_thread
            .spawn_result(async move { addr.send(Ping).await })
            .unwrap();
        assert_eq!(result, "pong");
    }
}
