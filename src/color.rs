use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::interval::Interval;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
        }
    }

    pub fn random_range(r: std::ops::Range<f64>) -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(r.clone()),
            g: rng.gen_range(r.clone()),
            b: rng.gen_range(r),
        }
    }

    pub fn write(&self, samples_per_pixel: usize) -> String {
        let scale = 1.0 / samples_per_pixel as f64;

        let c = self * scale;
        let r = linear_to_gamma(c.r);
        let g = linear_to_gamma(c.g);
        let b = linear_to_gamma(c.b);

        let intensity = Interval::new(0.0, 0.999);

        format!(
            "{} {} {}",
            (256.0 * intensity.clamp(r)).floor(),
            (256.0 * intensity.clamp(g)).floor(),
            (256.0 * intensity.clamp(b)).floor()
        )
    }
}

impl std::ops::Add for &Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Add<Color> for &Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Add<&Color> for Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::AddAssign<&Color> for Color {
    fn add_assign(&mut self, rhs: &Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl std::ops::Mul for &Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::Mul<Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        self * &rhs
    }
}

impl std::ops::Mul<&Color> for Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl std::ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl std::ops::Div<&Color> for f64 {
    type Output = Color;

    fn div(self, rhs: &Color) -> Self::Output {
        Color {
            r: self / rhs.r,
            g: self / rhs.g,
            b: self / rhs.b,
        }
    }
}

impl std::ops::Div<Color> for f64 {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        self / &rhs
    }
}

#[inline(always)]
fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
