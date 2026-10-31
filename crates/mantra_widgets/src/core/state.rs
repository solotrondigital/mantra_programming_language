//! Reactive state management primitives for Mantra Widgets.

use std::sync::{Arc, Mutex};

/// Lightweight observable value that triggers listeners when mutated.
#[derive(Clone)]
pub struct Observable<T: Clone> {
    value: Arc<Mutex<T>>,
    listeners: Arc<Mutex<Vec<Box<dyn Fn(&T) + Send + Sync>>>>,
}

impl<T: Clone + 'static> Observable<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial)),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get(&self) -> T {
        self.value.lock().unwrap().clone()
    }

    pub fn set(&self, new_val: T) {
        {
            let mut val_lock = self.value.lock().unwrap();
            *val_lock = new_val.clone();
        }
        self.notify(&new_val);
    }

    pub fn update<F: FnOnce(&mut T)>(&self, f: F) {
        let current_val = {
            let mut val_lock = self.value.lock().unwrap();
            f(&mut val_lock);
            val_lock.clone()
        };
        self.notify(&current_val);
    }

    pub fn subscribe<F: Fn(&T) + Send + Sync + 'static>(&self, listener: F) {
        self.listeners.lock().unwrap().push(Box::new(listener));
    }

    fn notify(&self, val: &T) {
        let listeners = self.listeners.lock().unwrap();
        for listener in listeners.iter() {
            listener(val);
        }
    }
}

/// Base trait for custom state controllers managing business logic and UI state.
pub trait StateController: Send + Sync {
    fn on_init(&mut self) {}
    fn on_dispose(&mut self) {}
}

