use std::{f64::consts::PI, sync::Arc};

use rand::Rng;

use crate::{
    color::Color, hittable::HitRecord, point3::Point3, ray::Ray, texture::Texture, vector3::Vector3,
};

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
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dielectric(d) => d.scatter(r_in, rec),
            Material::Isotropic(i) => i.scatter(r_in, rec),
            _ => None,
        }
    }

    pub fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Material::DiffuseLight(d) => d.emitted(u, v, p),
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        match self {
            Material::Lambertian(l) => l.scattering_pdf(r_in, rec, scattered),
            _ => todo!(),
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

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = Vector3::random_in_hemisphere(&rec.normal);

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal.clone();
        }

        let scattered = Ray::new(rec.p.clone(), scatter_dir, r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (2.0 * PI)
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.dir.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * Vector3::random_unit_vector(),
            r_in.time,
        );

        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((self.albedo.clone(), scattered))
        } else {
            None
        }
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

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
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
        Some((
            Color::new(1.0, 1.0, 1.0),
            Ray::new(rec.p.clone(), direction, r_in.time),
        ))
    }
}

impl Isotropic {
    pub fn new(albedo: Arc<Texture>) -> Self {
        Isotropic { albedo }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p.clone(), Vector3::random_unit_vector(), r_in.time),
        ))
    }
}

impl DiffuseLight {
    pub fn new(emit: Arc<Texture>) -> Self {
        DiffuseLight { emit }
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
