use std::sync::Arc;

use crate::{
    aabb::Aabb, interval::Interval, material::Material, point3::Point3, ray::Ray, vector3::Vector3,
};

use super::{HitRecord, Hittable, HittableList};

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vector3,
    v: Vector3,
    material: Arc<Material>,
    bbox: Aabb,
    normal: Vector3,
    d: f64,
    w: Vector3,
}

impl Quad {
    pub fn new(q: Point3, u: Vector3, v: Vector3, material: Arc<Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.unit_vector();
        let d = normal.dot(&(&q).into());
        let w = &n / n.dot(&n);
        let bbox = Aabb::new_from_points(&q, &(&q + &u + &v)).pad();
        Quad {
            q,
            u,
            v,
            material,
            bbox,
            normal,
            d,
            w,
        }
    }

    pub fn make_box(a: &Point3, b: &Point3, mat: Arc<Material>) -> Hittable {
        let mut sides = HittableList::new();

        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vector3::new(0.0, max.y - min.y, 0.0);
        let dz = Vector3::new(0.0, 0.0, max.z - min.z);

        sides.add(Hittable::Quad(Quad::new(
            Point3::new(min.x, min.y, max.z),
            dx.clone(),
            dy.clone(),
            Arc::clone(&mat),
        )));
        sides.add(Hittable::Quad(Quad::new(
            Point3::new(max.x, min.y, max.z),
            -&dz,
            dy.clone(),
            Arc::clone(&mat),
        )));
        sides.add(Hittable::Quad(Quad::new(
            Point3::new(max.x, min.y, min.z),
            -&dx,
            dy.clone(),
            Arc::clone(&mat),
        )));
        sides.add(Hittable::Quad(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dz.clone(),
            dy,
            Arc::clone(&mat),
        )));
        sides.add(Hittable::Quad(Quad::new(
            Point3::new(min.x, max.y, max.z),
            dx.clone(),
            -&dz,
            Arc::clone(&mat),
        )));
        sides.add(Hittable::Quad(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dx,
            dz,
            Arc::clone(&mat),
        )));

        Hittable::List(sides)
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&r.dir);

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(&(&r.orig).into())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let planar_hitpoint = &intersection - &self.q;
        let alpha = self.w.dot(&planar_hitpoint.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpoint));

        self.is_interior(alpha, beta).and_then(|(u, v)| {
            Some(HitRecord::new(
                r,
                &self.normal,
                intersection,
                t,
                u,
                v,
                Arc::clone(&self.material),
            ))
        })
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn is_interior(&self, a: f64, b: f64) -> Option<(f64, f64)> {
        if (a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b) {
            None
        } else {
            Some((a, b))
        }
    }
}
