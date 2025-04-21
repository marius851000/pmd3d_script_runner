use crate::gamedata::{Lock, Scene, Time, Update};
use crate::LockReason;
use crate::YieldResult;
use mlua::{UserData, UserDataMethods};
use std::sync::{atomic::AtomicBool, Arc, Mutex};

pub struct TASK {
    scene: Arc<Mutex<Scene>>,
}

impl TASK {
    pub fn new(scene: Arc<Mutex<Scene>>) -> Self {
        Self { scene }
    }
}

impl UserData for TASK {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("_Sleep", |_, this, time: Time| {
            let mut scene = this.scene.lock().unwrap();
            let abool = Arc::new(AtomicBool::new(false));
            scene.update(Update::AddLock(Lock::Wait(abool.clone(), time)));
            Ok(YieldResult::new(LockReason::new_abool(abool)))
        });
    }
}
