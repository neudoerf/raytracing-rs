use std::sync::Arc;

use image::RgbImage;

use crate::{color::Color, interval::Interval, perlin::Perlin, point3::Point3};

#[derive(Clone, Debug)]
pub struct Checker {
    inv_scale: f64,
    even: Arc<Texture>,
    odd: Arc<Texture>,
}

#[derive(Clone, Debug)]
pub struct Image {
    image: Arc<RgbImage>,
}

#[derive(Clone, Debug)]
pub struct Noise {
    noise: Perlin,
    scale: f64,
}

#[derive(Clone, Debug)]
pub enum Texture {
    SolidColor(Color),
    Checker(Checker),
    Image(Image),
    Noise(Noise),
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Texture::SolidColor(c) => c.clone(),
            Texture::Checker(checker) => checker.value(u, v, p),
            Texture::Image(image) => image.value(u, v, p),
            Texture::Noise(noise) => noise.value(u, v, p),
        }
    }
}

impl Checker {
    pub fn new(scale: f64, even: Arc<Texture>, odd: Arc<Texture>) -> Checker {
        Checker {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_solid(scale: f64, even: Color, odd: Color) -> Checker {
        Checker::new(
            scale,
            Arc::new(Texture::SolidColor(even)),
            Arc::new(Texture::SolidColor(odd)),
        )
    }

    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_int = (p.x * self.inv_scale).floor() as i64;
        let y_int = (p.y * self.inv_scale).floor() as i64;
        let z_int = (p.z * self.inv_scale).floor() as i64;
        let is_even = (x_int + y_int + z_int).checked_rem(2).unwrap() == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

impl Image {
    pub fn new(filename: &str) -> Image {
        Image {
            image: Arc::new(image::open(filename).unwrap().into_rgb8()),
        }
    }

    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.dimensions().0 <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.dimensions().0 as f64).floor() as u32;
        let j = (v * self.image.dimensions().1 as f64).floor() as u32;
        let pixel = self.image[(i, j)];

        let color_scale = 1.0 / 255.0;

        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

impl Noise {
    pub fn new(scale: f64) -> Noise {
        Noise {
            noise: Perlin::new(),
            scale,
        }
    }

    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let s = self.scale * p;
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (s.z + 10.0 * self.noise.turb(&s, None)).sin())
    }
}
