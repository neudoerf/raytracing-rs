use std::{f64::consts::PI, sync::Arc};

use rand::Rng;

use crate::{
    aabb::Aabb, interval::Interval, material::Material, onb::Onb, point3::Point3, ray::Ray,
    vector3::Vector3,
};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Sphere {
    center1: Point3,
    radius: f64,
    material: Arc<Material>,
    center_vec: Option<Vector3>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<Material>) -> Hittable {
        let rvec = Vector3::new(radius, radius, radius);
        Hittable::Sphere(Sphere {
            center1: center.clone(),
            radius,
            material,
            center_vec: None,
            bbox: Aabb::new_from_points(&(&center - &rvec), &(center + rvec)),
        })
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Arc<Material>,
    ) -> Hittable {
        let rvec = Vector3::new(radius, radius, radius);
        let center_vec = &center2 - &center1;
        Hittable::Sphere(Sphere {
            center1: center1.clone(),
            radius,
            material,
            center_vec: Some(center_vec),
            bbox: Aabb::new_from_aabbs(
                &Aabb::new_from_points(&(&center1 - &rvec), &(&center1 + &rvec)),
                &Aabb::new_from_points(&(&center2 - &rvec), &(&center2 + &rvec)),
            ),
        })
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let center = self
            .center_vec
            .as_ref()
            .and_then(|v| Some(&self.center1 + r.time * v));
        let center = center.as_ref().unwrap_or(&self.center1);
        let oc = &r.orig - center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (&p - center) / self.radius;
        let (u, v) = get_sphere_uv(&(&outward_normal).into());
        Some(HitRecord::new(
            r,
            &outward_normal,
            p,
            root,
            u,
            v,
            Arc::clone(&self.material),
        ))
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    pub fn pdf_value(&self, o: &Point3, v: &Vector3) -> f64 {
        self.hit(
            &Ray::new(o.clone(), v.clone(), 0.0),
            Interval::new(0.001, f64::MAX),
        )
        .and_then(|_| {
            let r_sq = self.radius * self.radius;
            let c_o_sq = (&self.center1 - o).length_squared();
            if r_sq > c_o_sq {
                Some(0.0)
            } else {
                let cos_theta_max =
                    (1.0 - self.radius * self.radius / (&self.center1 - o).length_squared()).sqrt();
                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

                Some(1.0 / solid_angle)
            }
        })
        .unwrap_or(0.0)
    }

    pub fn random(&self, o: &Point3) -> Vector3 {
        let dir = &self.center1 - o;
        let distance_squared = dir.length_squared();
        let uvw = Onb::new(&dir);
        uvw.local(&random_to_sphere(self.radius, distance_squared))
    }
}

fn random_to_sphere(radius: f64, distance_squared: f64) -> Vector3 {
    let mut rng = rand::thread_rng();
    let r1: f64 = rng.gen();
    let r2: f64 = rng.gen();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vector3::new(x, y, z)
}

fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;
    // eprintln!("{} {} {}", p.x, p.y, p.z);

    // eprintln!("{} {}", theta, phi);

    (phi / (2.0 * PI), theta / PI)
}

#[cfg(test)]
mod tests {
    use crate::{hittable::sphere::get_sphere_uv, point3::Point3};

    #[test]
    fn test_sphere_uv() {
        assert_eq!(get_sphere_uv(&Point3::new(1.0, 0.0, 0.0)), (0.5, 0.5));
        assert_eq!(get_sphere_uv(&Point3::new(0.0, 1.0, 0.0)), (0.5, 1.0));
        assert_eq!(get_sphere_uv(&Point3::new(0.0, 0.0, 1.0)), (0.25, 0.5));
        assert_eq!(get_sphere_uv(&Point3::new(-1.0, 0.0, 0.0)), (0.0, 0.5));
        assert_eq!(get_sphere_uv(&Point3::new(0.0, -1.0, 0.0)), (0.5, 0.0));
        assert_eq!(get_sphere_uv(&Point3::new(0.0, 0.0, -1.0)), (0.75, 0.5));
    }
}
