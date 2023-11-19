use serde::{Deserialize, Serialize};

use crate::{camera, color::Color, point3::Point3, vector3::Vector3};

#[derive(Serialize, Deserialize)]
pub struct Camera {
    image_width: u32,
    aspect_ratio: f64,
    look_from: Point3,
    look_at: Point3,
    v_up: Vector3,
    v_fov: f64,
    defocus_angle: f64,
    focus_dist: f64,
    samples_per_pixel: usize,
    max_depth: u32,
    background: Color,
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        look_from: Point3,
        look_at: Point3,
        v_up: Vector3,
        v_fov: f64,
        defocus_angle: f64,
        focus_dist: f64,
        samples_per_pixel: usize,
        max_depth: u32,
        background: Color,
    ) -> Self {
        Camera {
            image_width,
            aspect_ratio,
            look_from,
            look_at,
            v_up,
            v_fov,
            defocus_angle,
            focus_dist,
            samples_per_pixel,
            max_depth,
            background,
        }
    }
}

impl Into<camera::Camera> for Camera {
    fn into(self) -> camera::Camera {
        camera::Camera::new(
            self.image_width,
            self.aspect_ratio,
            self.look_from,
            self.look_at,
            self.v_up,
            self.v_fov,
            self.defocus_angle,
            self.focus_dist,
            self.samples_per_pixel,
            self.max_depth,
            self.background,
        )
    }
}
