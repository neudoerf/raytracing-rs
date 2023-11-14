use std::{
    env::{self},
    sync::Arc,
};

use camera::Camera;
use color::Color;
use hittable::{BvhNode, Hittable, HittableList, Sphere};
use material::{Dielectric, Lambertian, Material, Metal};
use point3::Point3;
use rand::Rng;
use texture::{Checker, Texture};
use vector3::Vector3;

use crate::{
    hittable::{ConstantMedium, Quad, RotateY, Translate},
    material::DiffuseLight,
    texture::{Image, Noise},
};

mod aabb;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod perlin;
mod point3;
mod ray;
mod texture;
mod vector3;

fn random_spheres(image_width: u32, samples_per_pixel: usize) -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let mut rng = rand::thread_rng();

    let checker = Arc::new(Texture::Checker(Checker::new_solid(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Material::Lambertian(Lambertian::new(checker))),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (&center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material =
                        Arc::new(Material::Lambertian(Lambertian::new_solid(albedo)));
                    let center2 = &center + Vector3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Hittable::Sphere(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Arc::new(Material::Metal(Metal::new(albedo, fuzz)));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Arc::new(Material::Dielectric(Dielectric::new(1.5)));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Arc::new(Material::Dielectric(Dielectric::new(1.5)));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.4, 0.2, 0.1,
    ))));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let bvh = BvhNode::new(&world.objects, 0, world.objects.len());
    let mut world = HittableList::new();
    world.add(bvh);

    let cam = Camera::new(
        image_width,
        16.0 / 9.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        0.6,
        10.0,
        samples_per_pixel,
        50,
        Color::new(0.7, 0.8, 1.0),
    );

    (world, cam)
}

fn two_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let checker = Arc::new(Texture::Checker(Checker::new_solid(
        0.8,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    world.add(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Material::Lambertian(Lambertian::new(Arc::clone(&checker)))),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Material::Lambertian(Lambertian::new(Arc::clone(&checker)))),
    ));

    let cam = Camera::new(
        400,
        16.0 / 9.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        0.6,
        10.0,
        100,
        50,
        Color::new(0.7, 0.8, 1.0),
    );

    (world, cam)
}

fn earth() -> (HittableList, Camera) {
    let earth_texture = Arc::new(Texture::Image(Image::new("earthmap.jpg")));
    let earth_surface = Arc::new(Material::Lambertian(Lambertian::new(earth_texture)));
    // let earth_surface = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
    //     0.0, 0.0, 0.0,
    // ))));
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    let cam = Camera::new(
        400,
        16.0 / 9.0,
        Point3::new(0.0, 0.0, 12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        0.0,
        1.0,
        100,
        50,
        Color::new(0.7, 0.8, 1.0),
    );

    let mut world = HittableList::new();
    world.add(globe);

    (world, cam)
}

fn two_perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let pertext = Arc::new(Texture::Noise(Noise::new(4.0)));
    let permat = Arc::new(Material::Lambertian(Lambertian::new(pertext)));
    // let permat = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
    //     0.5, 0.5, 0.5,
    // ))));

    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&permat),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&permat),
    ));

    let cam = Camera::new(
        400,
        16.0 / 9.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        0.6,
        10.0,
        100,
        50,
        Color::new(0.7, 0.8, 1.0),
    );

    (world, cam)
}

fn quads() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    // Materials
    let left_red = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        1.0, 0.2, 0.2,
    ))));
    let back_green = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.2, 1.0, 0.2,
    ))));
    let right_blue = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.2, 0.2, 1.0,
    ))));
    let upper_orange = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        1.0, 0.5, 0.0,
    ))));
    let lower_teal = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.2, 0.8, 0.8,
    ))));

    // Quads
    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vector3::new(0.0, 0.0, -4.0),
        Vector3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vector3::new(0.0, 0.0, 4.0),
        Vector3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let cam = Camera::new(
        400,
        1.0,
        Point3::new(0.0, 0.0, 9.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        80.0,
        0.0,
        1.0,
        100,
        50,
        Color::new(0.7, 0.8, 1.0),
    );

    (world, cam)
}

fn simple_light() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let pertext = Arc::new(Texture::Noise(Noise::new(4.0)));
    let permat = Arc::new(Material::Lambertian(Lambertian::new(pertext)));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&permat),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&permat),
    ));

    let difflight = Arc::new(Material::DiffuseLight(DiffuseLight::new(Arc::new(
        Texture::SolidColor(Color::new(4.0, 4.0, 4.0)),
    ))));
    world.add(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        Arc::clone(&difflight),
    ));
    world.add(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vector3::new(2.0, 0.0, 0.0),
        Vector3::new(0.0, 2.0, 0.0),
        difflight,
    ));

    let cam = Camera::new(
        400,
        16.0 / 9.0,
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        0.0,
        1.0,
        100,
        50,
        Color::new(0.0, 0.0, 0.0),
    );

    (world, cam)
}

fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let red = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.65, 0.05, 0.05,
    ))));
    let white = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.73, 0.73, 0.73,
    ))));
    let green = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.12, 0.45, 0.15,
    ))));
    let light = Arc::new(Material::DiffuseLight(DiffuseLight::new(Arc::new(
        Texture::SolidColor(Color::new(15.0, 15.0, 15.0)),
    ))));

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        Arc::clone(&white),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        Arc::clone(&white),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Arc::clone(&white),
    ));

    let box1 = Quad::make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    );
    let box1 = Hittable::RotateY(RotateY::new(Arc::new(box1), 15.0));
    let box1 = Hittable::Translate(Translate::new(
        Arc::new(box1),
        Vector3::new(265.0, 0.0, 295.0),
    ));
    world.add(box1);

    let box2 = Quad::make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Hittable::RotateY(RotateY::new(Arc::new(box2), -18.0));
    let box2 = Hittable::Translate(Translate::new(
        Arc::new(box2),
        Vector3::new(130.0, 0.0, 65.0),
    ));
    world.add(box2);

    let cam = Camera::new(
        600,
        1.0,
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        40.0,
        0.0,
        1.0,
        200,
        50,
        Color::new(0.0, 0.0, 0.0),
    );

    (world, cam)
}

fn cornell_smoke() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let red = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.65, 0.05, 0.05,
    ))));
    let white = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.73, 0.73, 0.73,
    ))));
    let green = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.12, 0.45, 0.15,
    ))));
    let light = Arc::new(Material::DiffuseLight(DiffuseLight::new(Arc::new(
        Texture::SolidColor(Color::new(7.0, 7.0, 7.0)),
    ))));

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vector3::new(330.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 305.0),
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        Arc::clone(&white),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        Arc::clone(&white),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Arc::clone(&white),
    ));

    let box1 = Quad::make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    );
    let box1 = Hittable::RotateY(RotateY::new(Arc::new(box1), 15.0));
    let box1 = Hittable::Translate(Translate::new(
        Arc::new(box1),
        Vector3::new(265.0, 0.0, 295.0),
    ));
    world.add(ConstantMedium::new(
        Arc::new(box1),
        0.01,
        Arc::new(Texture::SolidColor(Color::new(0.0, 0.0, 0.0))),
    ));

    let box2 = Quad::make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Hittable::RotateY(RotateY::new(Arc::new(box2), -18.0));
    let box2 = Hittable::Translate(Translate::new(
        Arc::new(box2),
        Vector3::new(130.0, 0.0, 65.0),
    ));
    world.add(ConstantMedium::new(
        Arc::new(box2),
        0.01,
        Arc::new(Texture::SolidColor(Color::new(1.0, 1.0, 1.0))),
    ));

    let cam = Camera::new(
        600,
        1.0,
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        40.0,
        0.0,
        1.0,
        400,
        50,
        Color::new(0.0, 0.0, 0.0),
    );

    (world, cam)
}

fn final_scene(
    image_width: u32,
    samples_per_pixel: usize,
    max_depth: u32,
) -> (HittableList, Camera) {
    let mut rng = rand::thread_rng();
    let mut boxes1: Vec<Hittable> = vec![];
    let ground = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.48, 0.83, 0.53,
    ))));
    let glass = Arc::new(Material::Dielectric(Dielectric::new(1.5)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let i = i as f64;
            let j = j as f64;
            let w = 100.0;
            let x0 = -1000.0 + i * w;
            let z0 = -1000.0 + j * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.push(Quad::make_box(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                Arc::clone(&ground),
            ));
        }
    }

    let mut world = HittableList::new();

    world.add(BvhNode::new(&boxes1, 0, boxes1.len()));

    let light = Arc::new(Material::DiffuseLight(DiffuseLight::new(Arc::new(
        Texture::SolidColor(Color::new(7.0, 7.0, 7.0)),
    ))));
    world.add(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vector3::new(300.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 265.0),
        light,
    ));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = &center1 + Vector3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.7, 0.3, 0.1,
    ))));
    world.add(Hittable::Sphere(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::clone(&glass),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0))),
    ));

    let boundary = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Arc::clone(&glass));
    world.add(boundary.clone());
    world.add(ConstantMedium::new(
        Arc::new(boundary),
        0.2,
        Arc::new(Texture::SolidColor(Color::new(0.2, 0.4, 0.9))),
    ));
    let boundary = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Arc::clone(&glass));
    world.add(ConstantMedium::new(
        Arc::new(boundary),
        0.0001,
        Arc::new(Texture::SolidColor(Color::new(1.0, 1.0, 1.0))),
    ));

    let emat = Arc::new(Material::Lambertian(Lambertian::new(Arc::new(
        Texture::Image(Image::new("earthmap.jpg")),
    ))));
    world.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));
    let pertext = Arc::new(Texture::Noise(Noise::new(0.1)));
    world.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Material::Lambertian(Lambertian::new(pertext))),
    ));

    let mut boxes2: Vec<Hittable> = vec![];
    let white = Arc::new(Material::Lambertian(Lambertian::new_solid(Color::new(
        0.73, 0.73, 0.73,
    ))));

    for _ in 0..1000 {
        boxes2.push(Sphere::new(
            Point3::random(0.0, 165.0),
            10.0,
            Arc::clone(&white),
        ));
    }

    world.add(Hittable::Translate(Translate::new(
        Arc::new(Hittable::RotateY(RotateY::new(
            Arc::new(BvhNode::new(&boxes2, 0, boxes2.len())),
            15.0,
        ))),
        Vector3::new(-100.0, 270.0, 395.0),
    )));

    let cam = Camera::new(
        image_width,
        1.0,
        Point3::new(478.0, 278.0, -600.0),
        Point3::new(278.0, 278.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        40.0,
        0.0,
        1.0,
        samples_per_pixel,
        max_depth,
        Color::new(0.0, 0.0, 0.0),
    );

    (world, cam)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene: usize = args.get(1).unwrap().parse().unwrap();
    let (world, cam) = match scene {
        1 => random_spheres(400, 100),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(1200, 10000, 40),
        _ => final_scene(600, 100, 50),
    };

    cam.render(&world);
}
