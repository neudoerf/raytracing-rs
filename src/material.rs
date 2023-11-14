use std::{f64::consts::PI, sync::Arc};

use rand::Rng;

use crate::{
    color::Color,
    hittable::HitRecord,
    pdf::{self, Pdf},
    point3::Point3,
    ray::Ray,
    texture::Texture,
    vector3::Vector3,
};

pub enum ScatterRecord<'a> {
    Pdf(Color, Pdf<'a>),
    Ray(Color, Ray),
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Arc<Texture>,
}

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

#[derive(Clone, Debug)]
pub struct Dielectric {
    ir: f64,
}

#[derive(Clone, Debug)]
pub struct Isotropic {
    albedo: Arc<Texture>,
}

#[derive(Clone, Debug)]
pub struct DiffuseLight {
    emit: Arc<Texture>,
}

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    Isotropic(Isotropic),
    DiffuseLight(DiffuseLight),
}

impl Material {
    pub fn scatter<'a>(&'a self, r_in: &Ray, rec: &'a HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dielectric(d) => d.scatter(r_in, rec),
            Material::Isotropic(i) => i.scatter(r_in, rec),
            _ => None,
        }
    }

    pub fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Material::DiffuseLight(d) => d.emitted(r_in, rec, u, v, p),
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        match self {
            Material::Lambertian(l) => l.scattering_pdf(r_in, rec, scattered),
            Material::Isotropic(i) => i.scattering_pdf(r_in, rec, scattered),
            _ => 0.0,
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Arc<Texture>) -> Self {
        Lambertian { albedo }
    }

    pub fn new_solid(albedo: Color) -> Self {
        Lambertian {
            albedo: Arc::new(Texture::SolidColor(albedo)),
        }
    }

    fn scatter<'a>(&'a self, _r_in: &Ray, rec: &'a HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        let pdf = pdf::Cosine::new(&rec.normal);
        Some(ScatterRecord::Pdf(attenuation, pdf))
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = rec.normal.dot(&scattered.dir.unit_vector());
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.dir.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * Vector3::random_in_unit_sphere(),
            r_in.time,
        );

        Some(ScatterRecord::Ray(self.albedo.clone(), scattered))
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.dir.unit_vector();
        let cos_theta = (-&unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, self.ir) > rand::thread_rng().gen() {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal, refraction_ratio)
            };
        Some(ScatterRecord::Ray(
            Color::new(1.0, 1.0, 1.0),
            Ray::new(rec.p.clone(), direction, r_in.time),
        ))
    }
}

impl Isotropic {
    pub fn new(albedo: Arc<Texture>) -> Self {
        Isotropic { albedo }
    }

    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::Pdf(
            self.albedo.value(rec.u, rec.v, &rec.p),
            pdf::Sphere::new(),
        ))
    }

    pub fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}

impl DiffuseLight {
    pub fn new(emit: Arc<Texture>) -> Self {
        DiffuseLight { emit }
    }

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            Color::new(0.0, 0.0, 0.0)
        } else {
            self.emit.value(u, v, p)
        }
    }
}
