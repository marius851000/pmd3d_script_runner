use crate::gamedata::Vec3;

#[derive(Debug)]
pub struct Chara {
    pub position: Vec3<f64>,
    _actor: String, //TODO: change with another data structure
}

impl Chara {
    pub fn new(actor: String) -> Self {
        Self {
            position: Vec3::default(),
            _actor: actor,
        }
    }

    pub fn set_position(&mut self, position: Vec3<f64>) {
        self.position = position;
    }
}
