use crate::gamedata::FaceType;

#[derive(Debug, Clone)]
pub struct Portrait {
    pub coord: (f64, f64), // the coordinate, as if it was on a 3ds screen. The renderer manage placing them at the good coordinate.
    pub actor: String,     // the actor id
    pub facetype: FaceType,
}
