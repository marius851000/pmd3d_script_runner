use crate::gamedata::{Scene, Speed, Update, Vec2_f64, Vec3_f64};
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
        methods.add_method("SetPosition", |_, this, position: Vec3_f64| {
            let mut scene = this.scene.lock().unwrap();
            scene.update(Update::SetPosition(this.id.clone(), position));
            Ok(())
        });
        methods.add_method("WalkTo", |_, this, (position, speed): (Vec2_f64, Speed)| {
            let mut scene = this.scene.lock().unwrap();
            scene.update(Update::WalkTo(this.id.clone(), position, speed));
            Ok(())
        })
    }
}
