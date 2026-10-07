#![deny(unsafe_code)]

use crate::sync::{Arc, Condvar, Mutex, thread};

#[cfg(feature = "loom")]
pub mod sync {
    pub use loom::sync::{Arc, Condvar, Mutex};
    pub use loom::thread;
}

#[cfg(not(feature = "loom"))]
pub mod sync {
    pub use std::sync::{Arc, Condvar, Mutex};
    pub use std::thread;
}

pub type Task = fn(i64);

pub struct ThreadPool {
    shared: Arc<Shared>,
    workers: Vec<thread::JoinHandle<()>>,
}

struct Shared {
    state: Mutex<State>,
    has_work: Condvar,
}

struct State {
    queue: Vec<i64>,
    shutting_down: bool,
}

impl ThreadPool {
    /// Create a pool with `worker_count` workers.
    ///
    /// # Panics
    ///
    /// Should panic when `worker_count == 0`.
    pub fn new(worker_count: usize, task: Task) -> Self {
        assert!(
            worker_count > 0,
            "worker_count must be greater than zero (0)"
        );
        let state: Mutex<State> = Mutex::new(State {
            queue: Vec::new(),
            shutting_down: false,
        });
        let has_work = Condvar::new();
        let shared: Arc<Shared> = Arc::new(Shared { state, has_work });
        let mut workers: Vec<thread::JoinHandle<()>> = Vec::new();
        for _ in 0..worker_count {
            let shared = Arc::clone(&shared);
            workers.push(thread::spawn(move || {
                worker_loop(shared, task);
            }))
        }
        ThreadPool { shared, workers }
    }

    /// Add one number to the work queue.
    pub fn execute(&self, num: i64) {
        {
            let mut state = self.shared.state.lock().unwrap();
            state.queue.push(num);
            self.shared.has_work.notify_one();
        }
    }

    /// Finish all queued work and stop all workers.
    pub fn shutdown(self) {
        {
            let mut state = self.shared.state.lock().unwrap();
            state.shutting_down = true;
            self.shared.has_work.notify_all();
        }
        for worker in self.workers {
            worker.join().expect("worker panicked");
        }
    }
}

fn worker_loop(shared: Arc<Shared>, task: Task) {
    loop {
        let curr_num = {
            let mut state = shared.state.lock().unwrap();
            while state.queue.is_empty() && !state.shutting_down {
                state = shared.has_work.wait(state).unwrap();
            }
            if let Some(num) = state.queue.pop() {
                Some(num)
            } else if state.shutting_down {
                None
            } else {
                continue;
            }
        };
        match curr_num {
            Some(num) => task(num),
            None => break,
        }
    }
}
