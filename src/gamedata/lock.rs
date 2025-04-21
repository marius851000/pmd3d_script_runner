use crate::gamedata::Time;
use std::sync::{atomic::AtomicBool, Arc};

#[derive(Debug, Clone)]
pub enum Lock {
    Wait(Arc<AtomicBool>, Time),
    WaitMove(Arc<AtomicBool>, String),
}
