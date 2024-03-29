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

//! This is a wrapper struct for `actix` that provides a single global dispatcher into several actix
//! `Arbiter` threads.
//!
//! Futures are spawned round-robin on a different arbiter.
//!
//! ## Usage
//! ```
//! use actix_system_threads::ActorSystem;
//! let system = ActorSystem::current();
//!
//! system.spawn(async move {
//!     // ...
//! });
//! let x = system.spawn_result(async move {
//!     10
//! });
//! assert_eq!(x, 10);
//! ```

use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;

use actix::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref THREAD: ActorSystem = {
        log::info!("Global actor system will start");
        ActorSystem::with_new_system()
    };
}

/// Wraps `actix` system and a number of `Arbiter` threads.
#[derive(Debug)]
pub struct ActorSystem {
    #[allow(dead_code)]
    system: System,
    arbiters: Vec<ArbiterHandle>,
    counter: Arc<AtomicUsize>,
}

impl ActorSystem {
    /// Get the current global actor system. This will initialize the system when called the first
    /// time.
    pub fn current() -> &'static Self {
        &THREAD
    }

    fn with_new_system() -> Self {
        let (tx, rx) = channel();
        let (sys_tx, sys_rx) = channel();

        let num_threads = 0; // num_cpus::get();
        log::info!("Starting actor system on {} threads", num_threads);
        std::thread::Builder::new()
            .name("actor-system-main".into())
            .spawn(move || {
                let system = System::new();
                let mut arbiters = vec![Arbiter::current()];
                for _ in 0..num_threads {
                    arbiters.push(Arbiter::new().handle());
                }
                tx.send(arbiters).unwrap();
                sys_tx.send(System::current()).unwrap();
                log::warn!("System started");
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

    /// Spawn a future on the next available arbiter thread. Returns immediately.
    #[allow(unused)]
    pub fn spawn<Fut>(&self, fut: Fut)
    where
        Fut: 'static + Send + Future<Output = ()>,
    {
        let target_id = self.next_arbiter_idx();
        log::debug!("Spawning task on arbiter_id={}", target_id);
        self.arbiters[target_id].spawn(fut);
    }

    /// Spawn a future on the next available arbiter and return the result. Blocks waiting on the
    /// result.
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
        A: Actor<Context = Context<A>> + Send,
    {
        Self::current().spawn_result(async move { actor.start() })
    }
}

impl Drop for ActorSystem {
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
        // In a block so that drop is tested as well
        {
            let _ = wisual_logger::try_init_from_env();
            let _actor_system_thread = ActorSystem::with_new_system();
        }
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
                log::info!("ping <-> pong");
                "pong".to_string()
            }
        }

        let _ = wisual_logger::try_init_from_env();
        let actor_system_thread = ActorSystem::with_new_system();

        let addr1: Addr<TestActor> =
            actor_system_thread.spawn_result(async { TestActor {}.start() });
        let addr2: Addr<TestActor> =
            actor_system_thread.spawn_result(async { TestActor {}.start() });
        let addr3: Addr<TestActor> =
            actor_system_thread.spawn_result(async { TestActor {}.start() });
        let result = actor_system_thread
            .spawn_result(async move { addr1.send(Ping).await })
            .unwrap();
        assert_eq!(result, "pong");
        let result = actor_system_thread
            .spawn_result(async move { addr2.send(Ping).await })
            .unwrap();
        assert_eq!(result, "pong");
        let result = actor_system_thread
            .spawn_result(async move { addr3.send(Ping).await })
            .unwrap();
        assert_eq!(result, "pong");
    }
}
