use std::f64::consts::PI;

use rand::Rng;

use crate::{hittable, onb::Onb, point3::Point3, vector3::Vector3};

pub enum Pdf<'a> {
    Sphere(Sphere),
    Cosine(Cosine),
    Hittable(Hittable<'a>),
    Mixture(Mixture<'a>),
}

impl Pdf<'_> {
    pub fn value(&self, dir: &Vector3) -> f64 {
        match self {
            Pdf::Sphere(s) => s.value(dir),
            Pdf::Cosine(c) => c.value(dir),
            Pdf::Hittable(h) => h.value(dir),
            Pdf::Mixture(m) => m.value(dir),
        }
    }

    pub fn generate(&self) -> Vector3 {
        match self {
            Pdf::Sphere(s) => s.generate(),
            Pdf::Cosine(c) => c.generate(),
            Pdf::Hittable(h) => h.generate(),
            Pdf::Mixture(m) => m.generate(),
        }
    }
}

pub struct Sphere {}

impl<'a> Sphere {
    pub fn new() -> Pdf<'a> {
        Pdf::Sphere(Sphere {})
    }

    fn value(&self, _dir: &Vector3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vector3 {
        Vector3::random_unit_vector()
    }
}

pub struct Cosine {
    uvw: Onb,
}

impl Cosine {
    pub fn new(w: &Vector3) -> Pdf {
        Pdf::Cosine(Cosine { uvw: Onb::new(w) })
    }

    fn value(&self, dir: &Vector3) -> f64 {
        let cos_theta = dir.unit_vector().dot(&self.uvw.w);
        0.0_f64.max(cos_theta / PI)
    }

    fn generate(&self) -> Vector3 {
        self.uvw.local(&Vector3::random_cosine_dir())
    }
}

pub struct Hittable<'a> {
    objects: &'a hittable::Hittable,
    orig: Point3,
}

impl Hittable<'_> {
    pub fn new(objects: &hittable::Hittable, orig: Point3) -> Pdf {
        Pdf::Hittable(Hittable { objects, orig })
    }

    fn value(&self, dir: &Vector3) -> f64 {
        self.objects.pdf_value(&self.orig, dir)
    }

    fn generate(&self) -> Vector3 {
        self.objects.random(&self.orig)
    }
}

pub struct Mixture<'a> {
    a: &'a Pdf<'a>,
    b: &'a Pdf<'a>,
}

impl Mixture<'_> {
    pub fn new<'a>(a: &'a Pdf<'a>, b: &'a Pdf<'a>) -> Pdf<'a> {
        Pdf::Mixture(Mixture { a, b })
    }

    fn value(&self, dir: &Vector3) -> f64 {
        0.5 * self.a.value(dir) + 0.5 * self.b.value(dir)
    }

    fn generate(&self) -> Vector3 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < 0.5 {
            self.a.generate()
        } else {
            self.b.generate()
        }
    }
}
