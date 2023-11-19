use serde::{Deserialize, Serialize};

use crate::vector3::Vector3;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Self {
        Vector3::random(min, max).into()
    }
}

impl From<&Vector3> for Point3 {
    fn from(value: &Vector3) -> Self {
        Point3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vector3> for Point3 {
    fn from(value: Vector3) -> Self {
        Point3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl std::fmt::Display for &Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::ops::Add<&Vector3> for &Point3 {
    type Output = Point3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<Vector3> for &Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Add<&Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::Add for &Point3 {
    type Output = Point3;

    fn add(self, rhs: Self) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<&Vector3> for Point3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::AddAssign<Vector3> for Point3 {
    fn add_assign(&mut self, rhs: Vector3) {
        *self += &rhs
    }
}

impl std::ops::Sub for &Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<Point3> for &Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Point3) -> Self::Output {
        self - &rhs
    }
}

impl std::ops::Sub<&Point3> for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: &Point3) -> Self::Output {
        &self - rhs
    }
}

impl std::ops::Sub for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl std::ops::Sub<&Vector3> for &Point3 {
    type Output = Point3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<Vector3> for &Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        self - &rhs
    }
}

impl std::ops::Sub<&Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        &self - rhs
    }
}

impl std::ops::Sub<Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        &self - &rhs
    }
}

impl std::ops::Mul<f64> for &Point3 {
    type Output = Point3;

    fn mul(self, rhs: f64) -> Self::Output {
        Point3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<&Point3> for f64 {
    type Output = Point3;

    fn mul(self, rhs: &Point3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::MulAssign<f64> for Point3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Index<usize> for &Point3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("indexed into point with index {}", index),
        }
    }
}
