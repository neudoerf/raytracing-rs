use rand::Rng;

use crate::{aabb::Aabb, interval::Interval, point3::Point3, ray::Ray, vector3::Vector3};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Hittable>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new(objects: Vec<Hittable>) -> Hittable {
        let bbox = objects
            .iter()
            .map(|h| h.bounding_box())
            .reduce(|acc, b| Aabb::new_from_aabbs(&acc, &b))
            .unwrap_or(Aabb::new_empty());
        Hittable::List(HittableList { objects, bbox })
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    pub fn pdf_value(&self, o: &Point3, v: &Vector3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        self.objects
            .iter()
            .map(|obj| obj.pdf_value(o, v) * weight)
            .sum()
    }

    pub fn random(&self, o: &Point3) -> Vector3 {
        let mut rng = rand::thread_rng();
        return self.objects[rng.gen_range(0..self.objects.len())].random(o);
    }
}
