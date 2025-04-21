use mlua::{FromLua, UserData};

#[derive(Debug, Clone, PartialEq, FromLua)]
pub enum FaceType {
    NORMAL,
}

impl UserData for FaceType {}
