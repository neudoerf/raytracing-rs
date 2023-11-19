mod camera;
mod geometry;
mod material;
mod texture;

use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use crate::hittable;
pub use camera::Camera;
pub use geometry::{ConstantMedium, Geometry, Quad, Sphere};
pub use material::Material;
pub use texture::Texture;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    world: Vec<Geometry>,
    lights: Vec<Geometry>,
    camera: Camera,
}

impl Scene {
    pub fn new(world: Vec<Geometry>, lights: Vec<Geometry>, camera: Camera) -> Self {
        Scene {
            world,
            lights,
            camera,
        }
    }
}

pub fn load(
    filename: &str,
) -> (
    hittable::Hittable,
    hittable::Hittable,
    crate::camera::Camera,
) {
    let scene: Scene = serde_json::from_reader(BufReader::new(
        File::open(filename).expect(&format!("failed to open file {}", filename)),
    ))
    .expect("failed to deserialize");

    let world: Vec<hittable::Hittable> = scene.world.into_iter().map(|obj| obj.into()).collect();
    let lights: Vec<hittable::Hittable> = scene.lights.into_iter().map(|obj| obj.into()).collect();

    let world = if world.len() == 1 {
        world[0].clone()
    } else {
        hittable::HittableList::new(world)
    };

    let lights = if lights.len() == 1 {
        lights[0].clone()
    } else {
        hittable::HittableList::new(lights)
    };

    (world, lights, scene.camera.into())
}
