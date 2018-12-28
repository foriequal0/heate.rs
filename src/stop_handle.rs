
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::atomic::Ordering;

#[derive(Clone)]
pub struct StopHandle {
    stop: Arc<AtomicBool>,
}

impl StopHandle {
    pub fn new() -> StopHandle {
        StopHandle { stop: Arc::new(AtomicBool::new(false)) }
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Release);
    }

    pub fn stopped(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }
}