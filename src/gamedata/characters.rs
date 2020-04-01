use crate::gamedata::Vec3;

//NOTE: all change on chara should be done throught Scene via update

pub struct Chara {
    position: Vec3<f32>,
    _actor: String, //TODO: change with another data structure
}

impl Chara {
    pub fn new(actor: String) -> Self {
        Self {
            position: Vec3::default(),
            _actor: actor,
        }
    }

    pub fn set_position(&mut self, position: Vec3<f32>) {
        self.position = position;
    }
}
