use std::sync::Arc;

use rand::Rng;

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::{Isotropic, Material},
    ray::Ray,
    texture::Texture,
    vector3::Vector3,
};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Box<Hittable>,
    neg_inv_density: f64,
    phase_func: Arc<Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Box<Hittable>, d: f64, a: Arc<Texture>) -> Hittable {
        Hittable::ConstantMedium(ConstantMedium {
            boundary,
            neg_inv_density: (-1.0 / d),
            phase_func: Arc::new(Material::Isotropic(Isotropic::new(a))),
        })
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let enable_debug = false;
        let mut rng = rand::thread_rng();
        let debugging = enable_debug && rng.gen::<f64>() < 0.00001;

        self.boundary
            .hit(r, Interval::universe())
            .and_then(|mut rec1| {
                self.boundary
                    .hit(r, Interval::new(rec1.t + 0.001, f64::MAX))
                    .and_then(|mut rec2| {
                        if debugging {
                            eprintln!("\nray_tmin={}, ray_tmax={}", rec1.t, rec2.t);
                        }

                        if rec1.t < ray_t.min {
                            rec1.t = ray_t.min;
                        }
                        if rec2.t > ray_t.max {
                            rec2.t = ray_t.max;
                        }

                        if rec1.t >= rec2.t {
                            None
                        } else {
                            if rec1.t < 0.0 {
                                rec1.t = 0.0;
                            }
                            let ray_length = r.dir.length();
                            let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                            let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();

                            if hit_distance > distance_inside_boundary {
                                None
                            } else {
                                let t = rec1.t + hit_distance / ray_length;
                                let p = r.at(t);
                                if debugging {
                                    eprintln!(
                                        "hit_distance={} rec.t={}, rec.p={}",
                                        hit_distance, t, &p
                                    );
                                }
                                Some(HitRecord {
                                    p,
                                    normal: Vector3::new(1.0, 0.0, 0.0),
                                    material: Arc::clone(&self.phase_func),
                                    t,
                                    u: 0.0,
                                    v: 0.0,
                                    front_face: true,
                                })
                            }
                        }
                    })
            })
    }

    pub fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
