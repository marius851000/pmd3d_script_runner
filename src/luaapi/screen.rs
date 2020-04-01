use crate::gamedata::{Color, Lock, Scene, Time, Update};
use crate::{LockReason, YieldResult};
use rlua::{UserData, UserDataMethods};
use std::sync::{atomic::AtomicBool, Arc, Mutex};

pub struct SCREEN {
    scene: Arc<Mutex<Scene>>,
    id: u16,
}

impl SCREEN {
    pub fn new(scene: Arc<Mutex<Scene>>, id: u16) -> Self {
        Self { scene, id }
    }
}
impl UserData for SCREEN {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("_FadeOut", |_, this, (duration, sleep): (Time, bool)| {
            //TODO: what happen when multiple fadeout are called ?
            let mut scene = this.scene.lock().unwrap();
            scene.update(Update::SetScreenColor(
                this.id,
                Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            ));
            scene.update(Update::TransitionScreenColor(
                this.id,
                duration.clone(),
                Color::black(),
            ));
            if sleep {
                let abool = Arc::new(AtomicBool::new(false));
                scene.update(Update::AddLock(Lock::Wait(abool.clone(), duration)));
                Ok(YieldResult::new(LockReason::new_abool(abool)))
            } else {
                Ok(YieldResult::new(LockReason::None))
            }
        });
    }
}
