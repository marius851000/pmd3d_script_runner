use crate::gamedata::{FaceType, Portrait, Scene, Update};
use crate::luaapi::SymAct;
use mlua::{UserData, UserDataMethods};
use std::sync::{Arc, Mutex};

pub struct WINDOW {
    scene: Arc<Mutex<Scene>>,
}

impl WINDOW {
    pub fn new(scene: Arc<Mutex<Scene>>) -> Self {
        Self { scene }
    }
}

impl UserData for WINDOW {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "DrawFace",
            |_, this, (x, y, actor, facetype): (f64, f64, SymAct, FaceType)| {
                let mut scene = this.scene.lock().unwrap();
                scene.update(Update::SetPortrait(Portrait {
                    coord: (x, y),
                    actor: actor.id,
                    facetype,
                }));
                Ok(())
            },
        );

        methods.add_method("RemoveFace", |_, this, (): ()| {
            let mut scene = this.scene.lock().unwrap();
            scene.update(Update::RemovePortrait);
            Ok(())
        });
    }
}
