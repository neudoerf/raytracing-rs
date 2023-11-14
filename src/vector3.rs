use rand::Rng;

use crate::{aabb::Aabb, point3::Point3};

#[derive(Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vector3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vector3::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vector3::random_in_unit_sphere().unit_vector()
    }

    pub fn reflect(&self, n: &Vector3) -> Vector3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(&self, n: &Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = (-(1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: &Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vector3 {
        let l = self.length();
        Vector3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

impl From<&Point3> for Vector3 {
    fn from(value: &Point3) -> Self {
        Vector3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Point3> for Vector3 {
    fn from(value: Point3) -> Self {
        Vector3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl std::ops::Add for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::Add<&Point3> for &Vector3 {
    type Output = Point3;

    fn add(self, rhs: &Point3) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<Point3> for &Vector3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Add<&Point3> for Vector3 {
    type Output = Point3;

    fn add(self, rhs: &Point3) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::Add<Point3> for Vector3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        self - &rhs
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        &self - &rhs
    }
}

impl std::ops::Sub<&Point3> for &Vector3 {
    type Output = Point3;

    fn sub(self, rhs: &Point3) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Point3> for &Vector3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Sub<Point3> for Vector3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        &rhs * self
    }
}

impl std::ops::Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl std::ops::Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Index<usize> for &Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("indexed into vector with index {}", index),
        }
    }
}

impl std::ops::Add<&Aabb> for &Vector3 {
    type Output = Aabb;

    fn add(self, rhs: &Aabb) -> Self::Output {
        rhs + self
    }
}

#[cfg(test)]
mod test {
    use super::Vector3;

    #[test]
    fn test_cross() {
        let x = &Vector3::new(1.0, 0.0, 0.0);
        let y = &Vector3::new(0.0, 1.0, 0.0);
        let z = &Vector3::new(0.0, 0.0, 1.0);

        assert_eq!(x.cross(&y), *z);
        assert_eq!(y.cross(&x), -z);
        assert_eq!(y.cross(&z), *x);
        assert_eq!(z.cross(&y), -x);
        assert_eq!(z.cross(&x), *y);
        assert_eq!(x.cross(&z), -y);
    }
}
