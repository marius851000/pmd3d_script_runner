#[derive(Default)]
pub struct Input {
    pub time_elapsed: f64,
}

impl Input {
    pub fn new(time: f64) -> Input {
        Input { time_elapsed: time }
    }
}
