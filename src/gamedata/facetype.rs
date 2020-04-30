use rlua::UserData;

#[derive(Debug, Clone, PartialEq)]
pub enum FaceType {
    NORMAL,
}

impl UserData for FaceType {}
