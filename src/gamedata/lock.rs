use crate::gamedata::{Scene, Time};
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc,
};

#[derive(Debug, Clone)]
pub enum Lock {
    Wait(Arc<AtomicBool>, Time),
    WaitMove(Arc<AtomicBool>, String),
}
