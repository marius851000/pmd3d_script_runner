use crate::gamedata::{Scene, Update, Vec3};
use rlua::{UserData, UserDataMethods};
use std::sync::{Arc, Mutex};

pub struct CH {
    scene: Arc<Mutex<Scene>>,
    id: String,
}

impl CH {
    pub fn new(scene: Arc<Mutex<Scene>>, id: String) -> CH {
        CH { scene, id }
    }
}
impl UserData for CH {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("SetPosition", |_, this, position: Vec3<f32>| {
            let mut scene = this.scene.lock().unwrap();
            scene.update(Update::SetPosition(this.id.clone(), position));
            Ok(())
        });
    }
}
