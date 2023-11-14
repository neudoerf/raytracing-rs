use crate::{point3::Point3, vector3::Vector3};

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vector3,
    pub time: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vector3, time: f64) -> Self {
        Ray { orig, dir, time }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.orig + t * &self.dir
    }
}
