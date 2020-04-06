use rlua::UserData;


/// Store an ingame speed, in unit per second
#[derive(Debug, Clone, Copy)]
pub struct Speed (pub f64);

impl Speed {
    pub fn new(speed: f64) -> Speed {
        Speed(speed)
    }
}

impl UserData for Speed {}
