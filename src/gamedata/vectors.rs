use mlua::{FromLua, UserData};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Default, FromLua)]
#[allow(non_camel_case_types)]
pub struct Vec3_f64 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl UserData for Vec3_f64 {}

impl Vec3_f64 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn to_vec2(&self) -> Vec2_f64 {
        Vec2_f64 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Add for Vec3_f64 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3_f64 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div for Vec3_f64 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Mul for Vec3_f64 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

#[test]
fn test_vec3_math() {
    let vec1 = Vec3_f64::new(0.5, 1.0, 2.0);
    let vec2 = Vec3_f64::new(1.0, 0.5, 2.0);
    assert_eq!(vec1 + vec2, Vec3_f64::new(0.5 + 1.0, 1.0 + 0.5, 2.0 + 2.0));
    assert_eq!(vec1 - vec2, Vec3_f64::new(0.5 - 1.0, 1.0 - 0.5, 2.0 - 2.0));
    assert_eq!(vec1 / vec2, Vec3_f64::new(0.5 / 1.0, 1.0 / 0.5, 2.0 / 2.0));
    assert_eq!(vec1 * vec2, Vec3_f64::new(0.5 * 1.0, 1.0 * 0.5, 2.0 * 2.0));
}

#[derive(Copy, Clone, Debug, PartialEq, Default, FromLua)]
#[allow(non_camel_case_types)]
pub struct Vec2_f64 {
    pub x: f64,
    pub y: f64,
}

impl UserData for Vec2_f64 {}

impl Vec2_f64 {
    pub fn distance(&self, other: &Vec2_f64) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec2_f64 {
        let sum = self.x.abs() + self.y.abs();
        Vec2_f64 {
            x: self.x / sum,
            y: self.y / sum,
        }
    }

    pub fn to_vec3(&self, z: f64) -> Vec3_f64 {
        Vec3_f64 {
            x: self.x,
            y: self.y,
            z,
        }
    }
}

impl Vec2_f64 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2_f64 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2_f64 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Div for Vec2_f64 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Mul for Vec2_f64 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

#[test]
fn test_vec2_math() {
    let vec1 = Vec2_f64::new(0.5, 1.0);
    let vec2 = Vec2_f64::new(1.0, 0.5);
    assert_eq!(vec1 + vec2, Vec2_f64::new(0.5 + 1.0, 1.0 + 0.5));
    assert_eq!(vec1 - vec2, Vec2_f64::new(0.5 - 1.0, 1.0 - 0.5));
    assert_eq!(vec1 / vec2, Vec2_f64::new(0.5 / 1.0, 1.0 / 0.5));
    assert_eq!(vec1 * vec2, Vec2_f64::new(0.5 * 1.0, 1.0 * 0.5));
}
