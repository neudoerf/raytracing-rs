use std::cmp::Ordering;

use rand::Rng;

use crate::{aabb::Aabb, interval::Interval, ray::Ray};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct BvhNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(src_objects: &Vec<Hittable>, start: usize, end: usize) -> Hittable {
        let mut objects = src_objects.clone();
        let mut rng = rand::thread_rng();
        let axis: usize = rng.gen_range(0..=2);

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if box_compare(&objects[start], &objects[start + 1], axis) == Ordering::Less {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            objects[start..end].sort_unstable_by(|a, b| box_compare(a, b, axis));
            let mid = start + object_span / 2;
            (
                BvhNode::new(&objects, start, mid),
                BvhNode::new(&objects, mid, end),
            )
        };

        let bbox = Aabb::new_from_aabbs(&left.bounding_box(), &right.bounding_box());

        Hittable::BvhNode(BvhNode {
            left: Box::new(left),
            right: Box::new(right),
            bbox,
        })
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, &ray_t) {
            return None;
        }

        self.left
            .hit(r, ray_t)
            .and_then(|rec| {
                self.right
                    .hit(r, Interval::new(ray_t.min, rec.t))
                    .or_else(|| Some(rec))
            })
            .or_else(|| self.right.hit(r, ray_t))
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

fn box_compare(a: &Hittable, b: &Hittable, axis: usize) -> Ordering {
    if a.bounding_box().axis(axis).min < b.bounding_box().axis(axis).min {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
