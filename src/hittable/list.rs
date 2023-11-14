use crate::{aabb::Aabb, interval::Interval, ray::Ray};

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
}
