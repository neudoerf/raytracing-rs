use std::sync::Arc;

use crate::{color::Color, material, scene::texture::Texture};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Material {
    Lambertian(Texture),
    Metal { color: Color, fuzz: f64 },
    Dielectric(f64),
    Isotropic(Texture),
    DiffuseLight(Texture),
}

impl Material {
    pub fn new_lambertian(t: Texture) -> Self {
        Material::Lambertian(t)
    }

    pub fn new_lamb_solid(color: Color) -> Self {
        Material::Lambertian(Texture::new_solid(color))
    }

    pub fn new_metal(color: Color, fuzz: f64) -> Self {
        Material::Metal { color, fuzz }
    }

    pub fn new_dielectric(ir: f64) -> Self {
        Material::Dielectric(ir)
    }

    pub fn new_isotropic(t: Texture) -> Self {
        Material::Isotropic(t)
    }

    pub fn new_diffuse_light(t: Texture) -> Self {
        Material::DiffuseLight(t)
    }
}

impl Into<crate::material::Material> for Material {
    fn into(self) -> crate::material::Material {
        match self {
            Material::Lambertian(t) => material::Lambertian::new(Arc::new(t.into())),
            Material::Metal { color, fuzz } => material::Metal::new(color, fuzz),
            Material::Dielectric(ir) => material::Dielectric::new(ir),
            Material::Isotropic(t) => material::Isotropic::new(Arc::new(t.into())),
            Material::DiffuseLight(t) => material::DiffuseLight::new(Arc::new(t.into())),
        }
    }
}
