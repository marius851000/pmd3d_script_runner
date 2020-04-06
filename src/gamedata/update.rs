use crate::gamedata::{Color, Lock, Time, Vec3_f64, Vec2_f64, Speed};

/// This enum store everything that can update the scene
#[derive(Debug, Clone)]
pub enum Update {
    /// id
    DelChara(String),
    /// id, actor
    AddChara(String, String),
    /// id, position
    SetPosition(String, Vec3_f64),
    /// id, posititon, speed
    WalkTo(String, Vec2_f64, Speed),
    /// Lock
    AddLock(Lock),
    /// time spent
    TimeSpent(Time),
    /// Set the color of a screen (by it's id), immediate
    SetScreenColor(u16, Color),
    /// Do a transition of the color of a screen (by it's id)
    TransitionScreenColor(u16, Time, Color),
}
