use crossbeam::queue::ArrayQueue;
use std::sync::{Arc, RwLock};

use crate::utils::log;

enum QueueState {
    // queue has not pushed any elements
    NotPushed,

    // queue has pushed at least one element
    Pushed,
}

pub struct Queue<T> {
    queue: Arc<RwLock<ArrayQueue<T>>>,
    state: QueueState,
    debug_name: &'static str,
}

impl<T> Queue<T> {
    pub fn new(cap: usize, debug_name: &'static str) -> Queue<T> {
        Queue {
            queue: Arc::new(RwLock::new(ArrayQueue::new(cap))),
            state: QueueState::NotPushed,
            debug_name,
        }
    }

    // return whether the queue can start reading
    pub fn is_pushed(&self) -> bool {
        !matches!(self.state, QueueState::NotPushed)
    }

    // return queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.read().unwrap().is_empty()
    }

    // push item to queue
    pub fn push(&mut self, value: T) -> bool {
        match self.queue.write() {
            Ok(queue) => {
                if queue.push(value).is_err() {
                    log::error(&format!("push item to queue:{} error", self.debug_name));
                    return false;
                }
                self.state = QueueState::Pushed;
                true
            }
            Err(err) => {
                log::error(&format!(
                    "get queue:{} lock error: {}",
                    self.debug_name, err
                ));
                false
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
        let mut queue: Queue<i32> = Queue::new(3, "test queue");

        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_pushed(), false);

        queue.push(1);
        queue.push(2);

        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.is_pushed(), true);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));

        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_pushed(), true);
    }

    #[test]
    fn test_multi_thread() {
        let queue: Arc<RwLock<Queue<i32>>> = Arc::new(RwLock::new(Queue::new(3, "test queue")));
        queue.write().unwrap().push(1);
        queue.write().unwrap().push(2);

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
