mod bvh;
mod constant_medium;
mod hitrecord;
mod list;
mod quad;
mod sphere;

use std::sync::Arc;

use crate::{aabb::Aabb, interval::Interval, point3::Point3, ray::Ray, vector3::Vector3};

pub use self::{
    bvh::BvhNode, constant_medium::ConstantMedium, hitrecord::HitRecord, list::HittableList,
    quad::Quad, sphere::Sphere,
};

#[derive(Clone)]
pub struct Translate {
    object: Arc<Hittable>,
    offset: Vector3,
    bbox: Aabb,
}

#[derive(Clone)]
pub struct RotateY {
    object: Arc<Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

#[derive(Clone)]
pub enum Hittable {
    Translate(Translate),
    RotateY(RotateY),
    Sphere(Sphere),
    Quad(Quad),
    ConstantMedium(ConstantMedium),
    List(HittableList),
    BvhNode(BvhNode),
}

impl Hittable {
    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Hittable::Translate(t) => t.hit(r, ray_t),
            Hittable::RotateY(rot) => rot.hit(r, ray_t),
            Hittable::Sphere(s) => s.hit(r, ray_t),
            Hittable::Quad(q) => q.hit(r, ray_t),
            Hittable::ConstantMedium(c) => c.hit(r, ray_t),
            Hittable::List(l) => l.hit(r, ray_t),
            Hittable::BvhNode(b) => b.hit(r, ray_t),
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        match self {
            Hittable::Translate(t) => t.bounding_box(),
            Hittable::RotateY(r) => r.bounding_box(),
            Hittable::Sphere(s) => s.bounding_box(),
            Hittable::Quad(q) => q.bounding_box(),
            Hittable::ConstantMedium(c) => c.bounding_box(),
            Hittable::List(l) => l.bounding_box(),
            Hittable::BvhNode(b) => b.bounding_box(),
        }
    }
}

impl Translate {
    pub fn new(object: Arc<Hittable>, offset: Vector3) -> Self {
        let bbox = &object.bounding_box() + &offset;
        Translate {
            object,
            offset,
            bbox,
        }
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(&r.orig - &self.offset, r.dir.clone(), r.time);

        self.object.hit(&offset_r, ray_t).and_then(|mut rec| {
            rec.p += &self.offset;
            Some(rec)
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

impl RotateY {
    pub fn new(object: Arc<Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    min = Point3::new(min.x.min(newx), min.y.min(y), min.z.min(newz));
                    max = Point3::new(max.x.max(newx), max.y.max(y), max.z.max(newz));
                }
            }
        }

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_from_points(&min, &max),
        }
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let origin = Point3::new(
            self.cos_theta * r.orig.x - self.sin_theta * r.orig.z,
            r.orig.y,
            self.sin_theta * r.orig.x + self.cos_theta * r.orig.z,
        );
        let dir = Vector3::new(
            self.cos_theta * r.dir.x - self.sin_theta * r.dir.z,
            r.dir.y,
            self.sin_theta * r.dir.x + self.cos_theta * r.dir.z,
        );
        let rotated_r = Ray::new(origin, dir, r.time);

        self.object.hit(&rotated_r, ray_t).and_then(|mut rec| {
            rec.p = Point3::new(
                self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                rec.p.y,
                -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
            );
            rec.normal = Vector3::new(
                self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                rec.normal.y,
                -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
            );
            Some(rec)
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
