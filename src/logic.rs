use crate::gamedata::{Scene, Time, Update};
use crate::luaapi::initialize_lua_environment;
use crate::transform_script;
use crate::Input;
use crate::RunningLua;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Logic {
    lua: RunningLua,
    pub scene: Arc<Mutex<Scene>>,
}

impl Logic {
    pub fn new(script: &str) -> Self {
        let mut lua = RunningLua::default();
        let scene = Arc::new(Mutex::new(Scene::default()));
        initialize_lua_environment(&lua, &scene);
        lua.load_script(&transform_script(script));
        Logic { lua, scene }
    }

    pub fn execute(&mut self, input: Input) {
        {
            let mut lock = self.scene.lock().unwrap();
            lock.update(Update::TimeSpent(Time::new(input.time_elapsed)));
        }
        self.lua.execute()
    }

    pub fn get_and_clear_updates(&mut self) -> Vec<Update> {
        let mut lock = self.scene.lock().unwrap();
        lock.get_and_clear_updates()
    }
}

#[test]
fn test_logic() {
    let mut logic = Logic::new("OBJECT_DynamicLoad(CHARA, \"HERO\", \"KIBAGO\")");
    logic.execute(Input::default());

    match &logic.get_and_clear_updates()[0] {
        Update::AddChara(id, specie) => {
            assert_eq!(&*id, "HERO");
            assert_eq!(&*specie, "KIBAGO");
        }
        _ => (),
    };

    assert!(logic.get_and_clear_updates().is_empty());
}
