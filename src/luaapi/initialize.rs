#![allow(clippy::trivially_copy_pass_by_ref)]
use mlua::Lua;

use crate::gamedata::{FaceType, Scene, Speed, Time, Vec2_f64, Vec3_f64};
use crate::luaapi::{SymAct, CH, CHARA, SCREEN, TASK, WINDOW};
use crate::RunningLua;
use std::fmt::Write;
use std::sync::{Arc, Mutex};

pub fn initialize_lua_environment(running_lua: &RunningLua, scene: &Arc<Mutex<Scene>>) {
    let lua = running_lua.lua();

    fn add_non_blocking_method(lua: &Lua, method_name: &str, argument_number: usize) {
        let mut arguments_part = String::new();
        if argument_number == 0 {
            arguments_part = "".into();
        } else {
            for arg_counter in 0..argument_number {
                write!(&mut arguments_part, "arg{}, ", arg_counter).unwrap();
            }
            arguments_part.pop();
            arguments_part.pop();
        };

        let code_to_load = &format!(
            "function(this{}{})
                this:{}({})
            end",
            if argument_number == 0 { "" } else { ", " },
            arguments_part,
            method_name,
            arguments_part
        );

        let function: mlua::Function = lua.load(code_to_load).eval().unwrap();
        let globals = lua.globals();
        globals
            .set(format!("OBJECT_{}", method_name), function)
            .unwrap();
    }

    fn add_blocking_method(lua: &Lua, method_name: &str, argument_number: usize) {
        let mut arguments_part = String::new();
        for arg_counter in 0..argument_number {
            write!(&mut arguments_part, "arg{}, ", arg_counter).unwrap();
        }
        arguments_part.pop();
        arguments_part.pop();
        let code_to_load = &format!(
            "function(this{}{})
                coroutine.yield(this:_{}({}))
            end",
            if argument_number == 0 { "" } else { ", " },
            arguments_part,
            method_name,
            arguments_part
        );
        println!("{}", code_to_load);
        let function: mlua::Function = lua.load(code_to_load).eval().unwrap();
        let globals = lua.globals();
        globals
            .set(format!("OBJECT_{}", method_name), function)
            .unwrap();
    }

    let globals = lua.globals();

    // debug function...
    let yammy_log = lua
        .create_function(move |_, content: String| {
            println!("logged from lua : {:?}", content);
            Ok(())
        })
        .unwrap();
    globals.set("yammy_log", yammy_log).unwrap();

    // add CHARA
    globals.set("CHARA", CHARA::new(scene.clone())).unwrap();
    // add CH
    let scene_clone = scene.clone();
    let CH_function = lua
        .create_function(move |_, id: String| Ok(CH::new(scene_clone.clone(), id)))
        .unwrap();
    globals.set("CH", CH_function).unwrap();
    // add SymAct
    let scene_clone = scene.clone();
    let SymAct_function = lua
        .create_function(move |_, id: String| Ok(SymAct::new(scene_clone.clone(), id)))
        .unwrap();
    globals.set("SymAct", SymAct_function).unwrap();
    // add Vector
    let Vector_function = lua
        .create_function(|_, (x, y, z): (f64, f64, f64)| Ok(Vec3_f64::new(x, y, z)))
        .unwrap();
    globals.set("Vector", Vector_function).unwrap();
    // add Vector2
    let Vector2_function = lua
        .create_function(|_, (x, y): (f64, f64)| Ok(Vec2_f64::new(x, y)))
        .unwrap();
    globals.set("Vector2", Vector2_function).unwrap();
    // add Speed
    let Speed_function = lua
        .create_function(|_, speed: f64| Ok(Speed::new(speed)))
        .unwrap();
    globals.set("Speed", Speed_function).unwrap();
    // add TimeSec
    let TimeSec_function = lua
        .create_function(|_, time_sec: f64| Ok(Time::new(time_sec)))
        .unwrap();
    globals.set("TimeSec", TimeSec_function).unwrap();
    // add TASK
    globals.set("TASK", TASK::new(scene.clone())).unwrap();
    // add SCREEN_A and SCREEN_B
    globals
        .set("SCREEN_A", SCREEN::new(scene.clone(), 0))
        .unwrap();
    globals
        .set("SCREEN_B", SCREEN::new(scene.clone(), 1))
        .unwrap();
    // add WINDOW
    globals.set("WINDOW", WINDOW::new(scene.clone())).unwrap();

    // add PORTRAIT_TYPE
    let portrait_table = lua.create_table().unwrap();
    portrait_table.set("NORMAL", FaceType::NORMAL).unwrap();
    globals.set("FACE_TYPE", portrait_table).unwrap();

    // objects method that may return
    add_non_blocking_method(lua, "DynamicRemove", 1);
    add_non_blocking_method(lua, "DynamicLoad", 2);
    add_non_blocking_method(lua, "SetPosition", 1);
    add_non_blocking_method(lua, "WalkTo", 2);
    add_non_blocking_method(lua, "DrawFace", 4);
    add_non_blocking_method(lua, "RemoveFace", 0);
    add_blocking_method(lua, "Sleep", 1);
    add_blocking_method(lua, "FadeOut", 2);
    add_blocking_method(lua, "WaitMove", 0);
}
