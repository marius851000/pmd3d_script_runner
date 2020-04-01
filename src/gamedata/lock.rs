use crate::gamedata::{Scene, Time};
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc,
};

#[derive(Debug, Clone)]
pub enum Lock {
    Wait(Arc<AtomicBool>, Time),
}

impl Lock {
    pub fn is_finished(&self, _scene: &Scene) -> bool {
        match self {
            Self::Wait(_, time) => time.get_time() <= 0.0,
        }
    }

    pub fn unlock(&mut self) {
        match self {
            Self::Wait(lock, _) => {
                lock.store(true, Relaxed);
            }
        }
    }
}
