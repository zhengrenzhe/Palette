use crossbeam::atomic::AtomicCell;
use crossbeam::queue::ArrayQueue;
use std::sync::{Arc, RwLock};

use crate::utils::log;

const NOT_PUSHED: &str = "NOT_PUSHED";
const PUSHED: &str = "PUSHED";

pub struct Queue<T> {
    queue: Arc<RwLock<ArrayQueue<T>>>,
    state: AtomicCell<&'static str>,
    // queue name for log
    debug_name: &'static str,
    all_push_done: AtomicCell<bool>,
}

impl<T> Queue<T> {
    pub fn new(cap: usize, debug_name: &'static str) -> Queue<T> {
        Queue {
            queue: Arc::new(RwLock::new(ArrayQueue::new(cap))),
            state: AtomicCell::new(NOT_PUSHED),
            debug_name,
            all_push_done: AtomicCell::new(false),
        }
    }

    // return whether the queue can start reading
    pub fn is_pushed(&self) -> bool {
        self.state.load() == PUSHED
    }

    // return queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.read().unwrap().is_empty()
    }

    // return all push action is finished
    pub fn is_all_push_done(&self) -> bool {
        self.all_push_done.load()
    }

    // set all push action is finished
    pub fn set_all_push_done(&self) {
        self.all_push_done.store(true);
    }

    // push item to queue
    pub fn push(&mut self, value: T) {
        match self.queue.write() {
            Ok(queue) => {
                if queue.push(value).is_err() {
                    log::error(&format!("push item to queue:{} error", self.debug_name));
                    return;
                }
                self.state.store(PUSHED)
            }
            Err(err) => {
                log::error(&format!(
                    "get queue:{} lock error: {}",
                    self.debug_name, err
                ));
            }
        }
    }

    // pop item from queue
    pub fn pop(&self) -> Option<T> {
        if let Ok(queue) = self.queue.read() {
            return queue.pop();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Barrier;
    use std::thread;

    #[test]
    fn test_single_queue() {
        let mut queue: Queue<i32> = Queue::new(2, "test queue");

        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_pushed(), false);
        assert_eq!(queue.is_all_push_done(), false);

        queue.push(1);
        queue.push(2);
        queue.set_all_push_done();

        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.is_pushed(), true);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));

        assert_eq!(queue.is_pushed(), true);
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_pushed(), true);
        assert_eq!(queue.is_all_push_done(), true);
    }

    #[test]
    fn test_multi_thread() {
        let queue: Arc<RwLock<Queue<i32>>> = Arc::new(RwLock::new(Queue::new(2, "test queue")));
        queue.write().unwrap().push(1);
        queue.write().unwrap().push(2);

        assert_eq!(queue.read().unwrap().is_empty(), false);
        assert_eq!(queue.read().unwrap().is_pushed(), true);
        assert_eq!(queue.read().unwrap().is_all_push_done(), false);

        let mut handles = Vec::new();
        let barrier = Arc::new(Barrier::new(2));

        for _ in 1..3 {
            let b = barrier.clone();
            let q = queue.clone();
            handles.push(thread::spawn(move || {
                b.wait();
                assert_eq!(q.read().unwrap().pop().is_some(), true);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(queue.read().unwrap().is_empty(), true);
        assert_eq!(queue.read().unwrap().is_pushed(), true);
    }
}
