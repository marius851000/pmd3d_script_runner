use crate::gamedata::Scene;
use mlua::{FromLua, UserData};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, FromLua)]
pub struct SymAct {
    scene: Arc<Mutex<Scene>>,
    pub id: String,
}

impl SymAct {
    pub fn new(scene: Arc<Mutex<Scene>>, id: String) -> SymAct {
        SymAct { scene, id }
    }
}

impl UserData for SymAct {}
