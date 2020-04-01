use rlua::UserData;
use std::ops::{Add, Div, Mul, Sub};

//TODO: the generic if pretty ugly. Ask around on how to simplify this.

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vec3<
    T: std::fmt::Debug
        + Default
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<
        T: std::fmt::Debug
            + Default
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
    > UserData for Vec3<T>
{
}

impl<
        T: std::fmt::Debug
            + Default
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
    > Vec3<T>
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<
        T: std::fmt::Debug
            + Default
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
    > Add for Vec3<T>
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<
        T: std::fmt::Debug
            + Default
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
    > Sub for Vec3<T>
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<
        T: std::fmt::Debug
            + Default
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
    > Div for Vec3<T>
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<
        T: std::fmt::Debug
            + Default
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
    > Mul for Vec3<T>
{
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
    let vec1 = Vec3::new(0.5, 1.0, 2.0);
    let vec2 = Vec3::new(1.0, 0.5, 2.0);
    assert_eq!(vec1 + vec2, Vec3::new(0.5 + 1.0, 1.0 + 0.5, 2.0 + 2.0));
    assert_eq!(vec1 - vec2, Vec3::new(0.5 - 1.0, 1.0 - 0.5, 2.0 - 2.0));
    assert_eq!(vec1 / vec2, Vec3::new(0.5 / 1.0, 1.0 / 0.5, 2.0 / 2.0));
    assert_eq!(vec1 * vec2, Vec3::new(0.5 * 1.0, 1.0 * 0.5, 2.0 * 2.0));
}
