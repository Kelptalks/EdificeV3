use std::sync::atomic::{AtomicU64, Ordering};

pub struct IdGen {
    counter: AtomicU64,
}

impl IdGen {
    pub const fn new() -> Self {
        Self { counter: AtomicU64::new(1) }
    }

    pub fn new_id(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}