use std::mem;

use crate::{interval::Interval, point3::Point3, ray::Ray, vector3::Vector3};

#[derive(Clone, Debug, PartialEq)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x, y, z }
    }

    pub fn new_empty() -> Aabb {
        Aabb {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn new_from_points(a: &Point3, b: &Point3) -> Aabb {
        Aabb {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        }
    }

    pub fn new_from_aabbs(box1: &Aabb, box2: &Aabb) -> Aabb {
        Aabb {
            x: Interval::new_from_interval(&box1.x, &box2.x),
            y: Interval::new_from_interval(&box1.y, &box2.y),
            z: Interval::new_from_interval(&box1.z, &box2.z),
        }
    }

    pub fn pad(&self) -> Aabb {
        let delta = 0.0001;
        let newx = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };
        let newy = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };
        let newz = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };
        Aabb {
            x: newx,
            y: newy,
            z: newz,
        }
    }

    pub fn axis(&self, n: usize) -> Interval {
        if n == 1 {
            self.y
        } else if n == 2 {
            self.z
        } else {
            self.x
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let mut ray_t = *ray_t;
        for a in 0..3 as usize {
            let inv_d = 1.0 / (&r.dir)[a];
            let orig = (&r.orig)[a];

            let mut t0 = (self.axis(a).min - orig) * inv_d;
            let mut t1 = (self.axis(a).max - orig) * inv_d;

            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.min {
                ray_t.min = t0
            }

            if t1 < ray_t.max {
                ray_t.max = t1;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}

impl std::ops::Add<&Vector3> for &Aabb {
    type Output = Aabb;

    fn add(self, rhs: &Vector3) -> Aabb {
        Aabb {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<Vector3> for &Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vector3) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Add<&Vector3> for Aabb {
    type Output = Aabb;

    fn add(self, rhs: &Vector3) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::Add<Vector3> for Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vector3) -> Self::Output {
        &self + &rhs
    }
}
