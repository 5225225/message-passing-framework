use crate::message::Pod;
use std::collections::VecDeque;
use std::sync::Mutex;

pub struct ThreadSafeQueue<T> {
    lock: Mutex<()>,
    deq_queue: VecDeque<T>,
}

impl<T: Pod> ThreadSafeQueue<T> {
    pub fn new() -> Self {
        Self {
            lock: Mutex::new(()),
            deq_queue: VecDeque::new(),
        }
    }

    pub fn front(&self) -> Option<&T> {
        if let Err(_) = self.lock.lock() {
            return None;
        }
        self.deq_queue.front()
    }

    pub fn back(&self) -> Option<&T> {
        if let Err(_) = self.lock.lock() {
            return None;
        }
        self.deq_queue.back()
    }

    pub fn push_back(&mut self, item: T) {
        if let Err(_) = self.lock.lock() {}
        self.deq_queue.push_back(item);
    }

    pub fn push_front(&mut self, item: T) {
        if let Err(_) = self.lock.lock() {}
        self.deq_queue.push_front(item);
    }

    pub fn clear(&mut self) {
        if let Err(_) = self.lock.lock() {}
        self.deq_queue.clear();
    }

    pub fn len(&mut self) -> usize {
        if let Err(_) = self.lock.lock() {}
        self.deq_queue.len()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Err(_) = self.lock.lock() {}
        self.deq_queue.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if let Err(_) = self.lock.lock() {}
        self.deq_queue.pop_back()
    }
}
