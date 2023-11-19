use std::env::{self};

use raytracing_rs::scene;

// fn random_spheres(image_width: u32, samples_per_pixel: usize) -> (Hittable, Hittable, Camera) {
//     let mut world: Vec<Hittable> = vec![];
//     let mut rng = rand::thread_rng();

//     let checker = Arc::new(Checker::new_solid(
//         0.32,
//         Color::new(0.2, 0.3, 0.1),
//         Color::new(0.9, 0.9, 0.9),
//     ));
//     world.push(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(Lambertian::new(checker)),
//     ));

//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat: f64 = rng.gen();
//             let center = Point3::new(
//                 a as f64 + 0.9 * rng.gen::<f64>(),
//                 0.2,
//                 b as f64 + 0.9 * rng.gen::<f64>(),
//             );

//             if (&center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
//                 if choose_mat < 0.8 {
//                     // diffuse
//                     let albedo = Color::random() * Color::random();
//                     let sphere_material = Arc::new(Lambertian::new_solid(albedo));
//                     let center2 = &center + Vector3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
//                     world.push(Hittable::Sphere(Sphere::new_moving(
//                         center,
//                         center2,
//                         0.2,
//                         sphere_material,
//                     )));
//                 } else if choose_mat < 0.95 {
//                     // metal
//                     let albedo = Color::random_range(0.5..1.0);
//                     let fuzz = rng.gen_range(0.0..0.5);
//                     let sphere_material = Arc::new(Metal::new(albedo, fuzz));
//                     world.push(Sphere::new(center, 0.2, sphere_material));
//                 } else {
//                     // glass
//                     let sphere_material = Arc::new(Dielectric::new(1.5));
//                     world.push(Sphere::new(center, 0.2, sphere_material));
//                 }
//             }
//         }
//     }

//     let material1 = Arc::new(Dielectric::new(1.5));
//     world.push(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

//     let material2 = Arc::new(Lambertian::new_solid(Color::new(0.4, 0.2, 0.1)));
//     world.push(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

//     let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
//     world.push(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

//     let world = BvhNode::new(&world, 0, world.len());

//     let cam = Camera::new(
//         image_width,
//         16.0 / 9.0,
//         Point3::new(13.0, 2.0, 3.0),
//         Point3::new(0.0, 0.0, 0.0),
//         Vector3::new(0.0, 1.0, 0.0),
//         20.0,
//         0.6,
//         10.0,
//         samples_per_pixel,
//         50,
//         Color::new(0.7, 0.8, 1.0),
//     );

//     (world, HittableList::new(vec![]), cam)
// }

// fn two_spheres() -> (Hittable, Camera) {
//     let mut world: Vec<Hittable> = vec![];
//     let checker = Arc::new(Checker::new_solid(
//         0.8,
//         Color::new(0.2, 0.3, 0.1),
//         Color::new(0.9, 0.9, 0.9),
//     ));

//     world.push(Sphere::new(
//         Point3::new(0.0, -10.0, 0.0),
//         10.0,
//         Arc::new(Lambertian::new(Arc::clone(&checker))),
//     ));
//     world.push(Sphere::new(
//         Point3::new(0.0, 10.0, 0.0),
//         10.0,
//         Arc::new(Lambertian::new(Arc::clone(&checker))),
//     ));

//     let cam = Camera::new(
//         400,
//         16.0 / 9.0,
//         Point3::new(13.0, 2.0, 3.0),
//         Point3::new(0.0, 0.0, 0.0),
//         Vector3::new(0.0, 1.0, 0.0),
//         20.0,
//         0.6,
//         10.0,
//         100,
//         50,
//         Color::new(0.7, 0.8, 1.0),
//     );

//     (HittableList::new(world), cam)
// }

// fn earth() -> (Hittable, Camera) {
//     let earth_texture = Arc::new(Image::new("earthmap.jpg"));
//     let earth_surface = Arc::new(Lambertian::new(earth_texture));
//     let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

//     let cam = Camera::new(
//         400,
//         16.0 / 9.0,
//         Point3::new(0.0, 0.0, 12.0),
//         Point3::new(0.0, 0.0, 0.0),
//         Vector3::new(0.0, 1.0, 0.0),
//         20.0,
//         0.0,
//         1.0,
//         100,
//         50,
//         Color::new(0.7, 0.8, 1.0),
//     );

//     (globe, cam)
// }

// fn two_perlin_spheres() -> (Hittable, Camera) {
//     let mut world: Vec<Hittable> = vec![];
//     let pertext = Arc::new(Noise::new(4.0));
//     let permat = Arc::new(Lambertian::new(pertext));

//     world.push(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::clone(&permat),
//     ));
//     world.push(Sphere::new(
//         Point3::new(0.0, 2.0, 0.0),
//         2.0,
//         Arc::clone(&permat),
//     ));

//     let cam = Camera::new(
//         400,
//         16.0 / 9.0,
//         Point3::new(13.0, 2.0, 3.0),
//         Point3::new(0.0, 0.0, 0.0),
//         Vector3::new(0.0, 1.0, 0.0),
//         20.0,
//         0.6,
//         10.0,
//         100,
//         50,
//         Color::new(0.7, 0.8, 1.0),
//     );

//     (HittableList::new(world), cam)
// }

// fn quads() -> (Hittable, Camera) {
//     let mut world: Vec<Hittable> = vec![];

//     // Materials
//     let left_red = Arc::new(Lambertian::new_solid(Color::new(1.0, 0.2, 0.2)));
//     let back_green = Arc::new(Lambertian::new_solid(Color::new(0.2, 1.0, 0.2)));
//     let right_blue = Arc::new(Lambertian::new_solid(Color::new(0.2, 0.2, 1.0)));
//     let upper_orange = Arc::new(Lambertian::new_solid(Color::new(1.0, 0.5, 0.0)));
//     let lower_teal = Arc::new(Lambertian::new_solid(Color::new(0.2, 0.8, 0.8)));

//     // Quads
//     world.push(Quad::new(
//         Point3::new(-3.0, -2.0, 5.0),
//         Vector3::new(0.0, 0.0, -4.0),
//         Vector3::new(0.0, 4.0, 0.0),
//         left_red,
//     ));
//     world.push(Quad::new(
//         Point3::new(-2.0, -2.0, 0.0),
//         Vector3::new(4.0, 0.0, 0.0),
//         Vector3::new(0.0, 4.0, 0.0),
//         back_green,
//     ));
//     world.push(Quad::new(
//         Point3::new(3.0, -2.0, 1.0),
//         Vector3::new(0.0, 0.0, 4.0),
//         Vector3::new(0.0, 4.0, 0.0),
//         right_blue,
//     ));
//     world.push(Quad::new(
//         Point3::new(-2.0, 3.0, 1.0),
//         Vector3::new(4.0, 0.0, 0.0),
//         Vector3::new(0.0, 0.0, 4.0),
//         upper_orange,
//     ));
//     world.push(Quad::new(
//         Point3::new(-2.0, -3.0, 5.0),
//         Vector3::new(4.0, 0.0, 0.0),
//         Vector3::new(0.0, 0.0, -4.0),
//         lower_teal,
//     ));

//     let cam = Camera::new(
//         400,
//         1.0,
//         Point3::new(0.0, 0.0, 9.0),
//         Point3::new(0.0, 0.0, 0.0),
//         Vector3::new(0.0, 1.0, 0.0),
//         80.0,
//         0.0,
//         1.0,
//         100,
//         50,
//         Color::new(0.7, 0.8, 1.0),
//     );

//     (HittableList::new(world), cam)
// }

// fn simple_light() -> (Hittable, Camera) {
//     let mut world: Vec<Hittable> = vec![];

//     let pertext = Arc::new(Noise::new(4.0));
//     let permat = Arc::new(Lambertian::new(pertext));
//     world.push(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::clone(&permat),
//     ));
//     world.push(Sphere::new(
//         Point3::new(0.0, 2.0, 0.0),
//         2.0,
//         Arc::clone(&permat),
//     ));

//     let difflight = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
//         4.0, 4.0, 4.0,
//     )))));
//     world.push(Sphere::new(
//         Point3::new(0.0, 7.0, 0.0),
//         2.0,
//         Arc::clone(&difflight),
//     ));
//     world.push(Quad::new(
//         Point3::new(3.0, 1.0, -2.0),
//         Vector3::new(2.0, 0.0, 0.0),
//         Vector3::new(0.0, 2.0, 0.0),
//         difflight,
//     ));

//     let cam = Camera::new(
//         400,
//         16.0 / 9.0,
//         Point3::new(26.0, 3.0, 6.0),
//         Point3::new(0.0, 2.0, 0.0),
//         Vector3::new(0.0, 1.0, 0.0),
//         20.0,
//         0.0,
//         1.0,
//         100,
//         50,
//         Color::new(0.0, 0.0, 0.0),
//     );

//     (HittableList::new(world), cam)
// }

// fn cornell_box() -> (Hittable, Hittable, Camera) {
//     let mut world: Vec<Hittable> = vec![];
//     let mut lights: Vec<Hittable> = vec![];

//     let red = Arc::new(Lambertian::new_solid(Color::new(0.65, 0.05, 0.05)));
//     let white = Arc::new(Lambertian::new_solid(Color::new(0.73, 0.73, 0.73)));
//     let green = Arc::new(Lambertian::new_solid(Color::new(0.12, 0.45, 0.15)));
//     let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
//         15.0, 15.0, 15.0,
//     )))));

//     world.push(Quad::new(
//         Point3::new(555.0, 0.0, 0.0),
//         Vector3::new(0.0, 0.0, 555.0),
//         Vector3::new(0.0, 555.0, 0.0),
//         green,
//     ));
//     world.push(Quad::new(
//         Point3::new(0.0, 0.0, 555.0),
//         Vector3::new(0.0, 0.0, -555.0),
//         Vector3::new(0.0, 555.0, 0.0),
//         red,
//     ));
//     world.push(Quad::new(
//         Point3::new(0.0, 555.0, 0.0),
//         Vector3::new(555.0, 0.0, 0.0),
//         Vector3::new(0.0, 0.0, 555.0),
//         Arc::clone(&white),
//     ));
//     world.push(Quad::new(
//         Point3::new(0.0, 0.0, 555.0),
//         Vector3::new(555.0, 0.0, 0.0),
//         Vector3::new(0.0, 0.0, -555.0),
//         Arc::clone(&white),
//     ));
//     world.push(Quad::new(
//         Point3::new(555.0, 0.0, 555.0),
//         Vector3::new(-555.0, 0.0, 0.0),
//         Vector3::new(0.0, 555.0, 0.0),
//         Arc::clone(&white),
//     ));

//     let box1 = Quad::make_box(
//         &Point3::new(0.0, 0.0, 0.0),
//         &Point3::new(165.0, 330.0, 165.0),
//         Arc::clone(&white),
//     );
//     let box1 = Hittable::RotateY(RotateY::new(Box::new(box1), 15.0));
//     let box1 = Hittable::Translate(Translate::new(
//         Box::new(box1),
//         Vector3::new(265.0, 0.0, 295.0),
//     ));
//     world.push(box1);

//     // let box2 = Quad::make_box(
//     //     &Point3::new(0.0, 0.0, 0.0),
//     //     &Point3::new(165.0, 165.0, 165.0),
//     //     white,
//     // );
//     // let box2 = Hittable::RotateY(RotateY::new(Box::new(box2), -18.0));
//     // let box2 = Hittable::Translate(Translate::new(
//     //     Box::new(box2),
//     //     Vector3::new(130.0, 0.0, 65.0),
//     // ));
//     // world.push(box2);

//     let glass = Arc::new(Dielectric::new(1.5));
//     let glass_sphere = Sphere::new(Point3::new(190.0, 90.0, 190.0), 90.0, glass);
//     world.push(glass_sphere.clone());
//     lights.push(glass_sphere);

//     let light = Quad::new(
//         Point3::new(213.0, 554.0, 227.0),
//         Vector3::new(130.0, 0.0, 0.0),
//         Vector3::new(0.0, 0.0, 105.0),
//         light,
//     );
//     world.push(light.clone());
//     lights.push(light);

//     let cam = Camera::new(
//         600,
//         1.0,
//         Point3::new(278.0, 278.0, -800.0),
//         Point3::new(278.0, 278.0, 0.0),
//         Vector3::new(0.0, 1.0, 0.0),
//         40.0,
//         0.0,
//         1.0,
//         1000,
//         50,
//         Color::new(0.0, 0.0, 0.0),
//     );

//     (HittableList::new(world), HittableList::new(lights), cam)
// }

// fn cornell_smoke() -> (Hittable, Hittable, Camera) {
//     let mut world: Vec<Hittable> = vec![];

//     let red = Arc::new(Lambertian::new_solid(Color::new(0.65, 0.05, 0.05)));
//     let white = Arc::new(Lambertian::new_solid(Color::new(0.73, 0.73, 0.73)));
//     let green = Arc::new(Lambertian::new_solid(Color::new(0.12, 0.45, 0.15)));
//     let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
//         7.0, 7.0, 7.0,
//     )))));

//     world.push(Quad::new(
//         Point3::new(555.0, 0.0, 0.0),
//         Vector3::new(0.0, 555.0, 0.0),
//         Vector3::new(0.0, 0.0, 555.0),
//         green,
//     ));
//     world.push(Quad::new(
//         Point3::new(0.0, 0.0, 0.0),
//         Vector3::new(0.0, 555.0, 0.0),
//         Vector3::new(0.0, 0.0, 555.0),
//         red,
//     ));
//     world.push(Quad::new(
//         Point3::new(0.0, 0.0, 0.0),
//         Vector3::new(555.0, 0.0, 0.0),
//         Vector3::new(0.0, 0.0, 555.0),
//         Arc::clone(&white),
//     ));
//     world.push(Quad::new(
//         Point3::new(555.0, 555.0, 555.0),
//         Vector3::new(-555.0, 0.0, 0.0),
//         Vector3::new(0.0, 0.0, -555.0),
//         Arc::clone(&white),
//     ));
//     world.push(Quad::new(
//         Point3::new(0.0, 0.0, 555.0),
//         Vector3::new(555.0, 0.0, 0.0),
//         Vector3::new(0.0, 555.0, 0.0),
//         Arc::clone(&white),
//     ));

//     let light = Quad::new(
//         Point3::new(113.0, 554.0, 127.0),
//         Vector3::new(330.0, 0.0, 0.0),
//         Vector3::new(0.0, 0.0, 305.0),
//         light,
//     );
//     world.push(light.clone());

//     let box1 = Quad::make_box(
//         &Point3::new(0.0, 0.0, 0.0),
//         &Point3::new(165.0, 330.0, 165.0),
//         Arc::clone(&white),
//     );
//     let box1 = Hittable::RotateY(RotateY::new(Box::new(box1), 15.0));
//     let box1 = Hittable::Translate(Translate::new(
//         Box::new(box1),
//         Vector3::new(265.0, 0.0, 295.0),
//     ));
//     world.push(ConstantMedium::new(
//         Box::new(box1),
//         0.01,
//         Arc::new(SolidColor::new(Color::new(0.0, 0.0, 0.0))),
//     ));

//     let box2 = Quad::make_box(
//         &Point3::new(0.0, 0.0, 0.0),
//         &Point3::new(165.0, 165.0, 165.0),
//         white,
//     );
//     let box2 = Hittable::RotateY(RotateY::new(Box::new(box2), -18.0));
//     let box2 = Hittable::Translate(Translate::new(
//         Box::new(box2),
//         Vector3::new(130.0, 0.0, 65.0),
//     ));
//     world.push(ConstantMedium::new(
//         Box::new(box2),
//         0.01,
//         Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0))),
//     ));

//     let cam = Camera::new(
//         600,
//         1.0,
//         Point3::new(278.0, 278.0, -800.0),
//         Point3::new(278.0, 278.0, 0.0),
//         Vector3::new(0.0, 1.0, 0.0),
//         40.0,
//         0.0,
//         1.0,
//         400,
//         50,
//         Color::new(0.0, 0.0, 0.0),
//     );

//     (HittableList::new(world), light, cam)
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene_file: &str = args.get(1).unwrap();
    let (world, lights, camera) = scene::load(scene_file);

    // let (world, lights, cam) = match scene {
    //     1 => random_spheres(400, 100),
    //     // 2 => two_spheres(),
    //     // 3 => earth(),
    //     // 4 => two_perlin_spheres(),
    //     // 5 => quads(),
    //     // 6 => simple_light(),
    //     7 => cornell_box(),
    //     8 => cornell_smoke(),
    //     9 => final_scene(1200, 10000, 40),
    //     10 => final_scene(600, 100, 50),
    //     _ => panic!(),
    // };

    camera.render(&world, &lights);
}
