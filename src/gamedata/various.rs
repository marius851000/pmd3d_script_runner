use mlua::{FromLua, UserData};
use std::cmp::Ordering;
use std::ops::{AddAssign, SubAssign};

/// time, in second
#[derive(Debug, Clone, PartialEq, Copy, FromLua)]
pub struct Time(pub f64);

impl Time {
    pub fn new(duration: f64) -> Self {
        Self(duration)
    }

    /// Return the time contained by this structure, in second
    pub fn get_time(self) -> f64 {
        self.0
    }

    /// Define the time contained by this structure, in second
    pub fn set_time(&mut self, duration: f64) {
        self.0 = duration
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl UserData for Time {}

#[test]
fn test_time() {
    let mut time = Time::new(2.0);
    time -= Time::new(3.0);
    assert_eq!(time.get_time(), 2.0 - 3.0);
}
