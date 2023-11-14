use std::sync::Arc;

use crate::{material::Material, point3::Point3, ray::Ray, vector3::Vector3};

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub material: Arc<Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        r: &Ray,
        outward_normal: &Vector3,
        p: Point3,
        t: f64,
        u: f64,
        v: f64,
        material: Arc<Material>,
    ) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        }
    }
}
