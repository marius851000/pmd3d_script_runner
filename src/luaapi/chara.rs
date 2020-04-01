use crate::gamedata::{Scene, Update};
use rlua::{UserData, UserDataMethods};
use std::sync::{Arc, Mutex};

pub struct CHARA {
    scene: Arc<Mutex<Scene>>,
}

impl CHARA {
    pub fn new(scene: Arc<Mutex<Scene>>) -> Self {
        Self { scene }
    }
}

impl UserData for CHARA {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("DynamicLoad", |_, this, (id, specie): (String, String)| {
            let mut scene = this.scene.lock().unwrap();
            scene.update(Update::AddChara(id, specie));
            Ok(())
        });

        methods.add_method("DynamicRemove", |_, this, id: String| {
            let mut scene = this.scene.lock().unwrap();
            scene.update(Update::DelChara(id));
            Ok(())
        });
    }
}
