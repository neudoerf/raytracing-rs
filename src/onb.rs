use crate::vector3::Vector3;

pub struct Onb {
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
}

impl Onb {
    pub fn new(w: &Vector3) -> Self {
        let unit_w = w.unit_vector();
        let a = if unit_w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        let v = unit_w.cross(&a).unit_vector();
        let u = unit_w.cross(&v);
        Onb { u, v, w: unit_w }
    }

    pub fn local(&self, a: &Vector3) -> Vector3 {
        a.x * &self.u + a.y * &self.v + a.z * &self.w
    }

    pub fn local_axes(&self, a: f64, b: f64, c: f64) -> Vector3 {
        a * &self.u + b * &self.v + c * &self.v
    }
}

impl std::ops::Index<usize> for Onb {
    type Output = Vector3;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.u
        } else if index == 1 {
            &self.v
        } else if index == 2 {
            &self.w
        } else {
            panic!("indexed into Onb w/ index > 2")
        }
    }
}
