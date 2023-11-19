use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{hittable, point3::Point3, vector3::Vector3};

use super::{material::Material, texture::Texture};

#[derive(Clone, Serialize, Deserialize)]
pub enum Geometry {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    Quad(Quad),
    RectPrism(RectPrism),
    ConstantMedium(ConstantMedium),
    Translate(Translate),
    RotateY(RotateY),
    List(Vec<Geometry>),
    Bvh(Vec<Geometry>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Translate {
    object: Box<Geometry>,
    offset: Vector3,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RotateY {
    object: Box<Geometry>,
    angle: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RectPrism {
    a: Point3,
    b: Point3,
    material: Material,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConstantMedium {
    boundary: Box<Geometry>,
    density: f64,
    phase_func: Texture,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MovingSphere {
    center1: Point3,
    center2: Point3,
    radius: f64,
    material: Material,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Quad {
    q: Point3,
    u: Vector3,
    v: Vector3,
    material: Material,
}

impl Geometry {
    pub fn new_sphere(center: Point3, radius: f64, material: Material) -> Self {
        Geometry::Sphere(Sphere {
            center,
            radius,
            material,
        })
    }

    pub fn new_moving_sphere(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Material,
    ) -> Self {
        Geometry::MovingSphere(MovingSphere {
            center1,
            center2,
            radius,
            material,
        })
    }

    pub fn new_quad(q: Point3, u: Vector3, v: Vector3, material: Material) -> Self {
        Geometry::Quad(Quad { q, u, v, material })
    }

    pub fn new_box(a: Point3, b: Point3, material: Material) -> Self {
        Geometry::RectPrism(RectPrism { a, b, material })
    }

    pub fn new_constant_medium(boundary: Geometry, density: f64, phase_func: Texture) -> Self {
        Geometry::ConstantMedium(ConstantMedium {
            boundary: Box::new(boundary),
            density,
            phase_func,
        })
    }

    pub fn new_translate(object: Geometry, offset: Vector3) -> Self {
        Geometry::Translate(Translate {
            object: Box::new(object),
            offset,
        })
    }

    pub fn new_rotate_y(object: Geometry, angle: f64) -> Self {
        Geometry::RotateY(RotateY {
            object: Box::new(object),
            angle,
        })
    }

    pub fn new_list(list: Vec<Geometry>) -> Self {
        Geometry::List(list)
    }

    pub fn new_bvh(list: Vec<Geometry>) -> Self {
        Geometry::Bvh(list)
    }
}

impl Into<hittable::Hittable> for Geometry {
    fn into(self) -> hittable::Hittable {
        match self {
            Geometry::Sphere(s) => {
                hittable::Sphere::new(s.center, s.radius, Arc::new(s.material.into()))
            }
            Geometry::MovingSphere(ms) => hittable::Sphere::new_moving(
                ms.center1,
                ms.center2,
                ms.radius,
                Arc::new(ms.material.into()),
            ),
            Geometry::Quad(q) => hittable::Quad::new(q.q, q.u, q.v, Arc::new(q.material.into())),
            Geometry::RectPrism(r) => {
                hittable::Quad::make_box(&r.a, &r.b, Arc::new(r.material.into()))
            }
            Geometry::ConstantMedium(cm) => hittable::ConstantMedium::new(
                Box::new((*cm.boundary).into()),
                cm.density,
                Arc::new(cm.phase_func.into()),
            ),
            Geometry::Translate(t) => {
                hittable::Translate::new(Box::new((*t.object).into()), t.offset)
            }
            Geometry::RotateY(r) => hittable::RotateY::new(Box::new((*r.object).into()), r.angle),
            Geometry::List(l) => {
                hittable::HittableList::new(l.into_iter().map(|h| h.into()).collect())
            }
            Geometry::Bvh(b) => {
                let l = b.len();
                hittable::BvhNode::new(&b.into_iter().map(|h| h.into()).collect(), 0, l)
            }
        }
    }
}
