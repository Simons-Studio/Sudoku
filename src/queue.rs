//! Create a Queue structure so that multiple consumers can read from it

use std::sync::Mutex;

pub struct Queue<T> {
    data: Mutex<Vec<T>>,
}

impl<T> Queue<T> {
    pub const fn new(value: Vec<T>) -> Queue<T> {
        Queue {
            data: Mutex::new(value),
        }
    }

    fn pop(&mut self) -> Option<T> {
        let mut queue = self.data.lock().unwrap();
        queue.pop()
    }

    fn push(&mut self, value: T) {
        let mut queue = self.data.lock().unwrap();
        queue.push(value);
    }
}
