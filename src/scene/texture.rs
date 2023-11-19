use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{color::Color, texture};

#[derive(Clone, Serialize, Deserialize)]
pub enum Texture {
    SolidColor(Color),
    Checker {
        scale: f64,
        even: Box<Texture>,
        odd: Box<Texture>,
    },
    Image(String),
    Noise(f64),
}

impl Texture {
    pub fn new_solid(color: Color) -> Self {
        Texture::SolidColor(color)
    }

    pub fn new_checker(scale: f64, even: Texture, odd: Texture) -> Self {
        Texture::Checker {
            scale,
            even: Box::new(even),
            odd: Box::new(odd),
        }
    }

    pub fn new_image(filename: &str) -> Self {
        Texture::Image(filename.to_owned())
    }

    pub fn new_noise(scale: f64) -> Self {
        Texture::Noise(scale)
    }
}

impl Into<crate::texture::Texture> for Texture {
    fn into(self) -> crate::texture::Texture {
        match self {
            Texture::SolidColor(c) => texture::SolidColor::new(c),
            Texture::Checker { scale, even, odd } => {
                texture::Checker::new(scale, Arc::new((*even).into()), Arc::new((*odd).into()))
            }
            Texture::Image(filename) => texture::Image::new(&filename),
            Texture::Noise(scale) => texture::Noise::new(scale),
        }
    }
}
