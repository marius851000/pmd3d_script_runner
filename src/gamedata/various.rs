use rlua::UserData;
use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Time {
    duration: f64,
}

impl Time {
    pub fn new(duration: f64) -> Self {
        Self { duration }
    }

    /// Return the time contained by this structure, in second
    pub fn get_time(&self) -> f64 {
        self.duration
    }

    /// Define the time contained by this structure, in second
    pub fn set_time(&mut self, duration: f64) {
        self.duration = duration
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, other: Self) {
        self.duration -= other.duration
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Self) {
        self.duration += other.duration
    }
}

impl UserData for Time {}

#[test]
fn test_time() {
    let mut time = Time::new(2.0);
    time -= Time::new(3.0);
    assert_eq!(time.get_time(), 2.0 - 3.0);
}
