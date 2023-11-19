use rand::Rng;
use raytracing_rs::{
    color::Color,
    point3::Point3,
    scene::{Camera, Geometry, Material, Scene, Texture},
    vector3::Vector3,
};

#[allow(dead_code)]
fn final_scene(image_width: u32, samples_per_pixel: usize, max_depth: u32) -> Scene {
    let mut rng = rand::thread_rng();
    let mut boxes1: Vec<Geometry> = vec![];
    let ground = Material::new_lambertian(Texture::new_solid(Color::new(0.48, 0.83, 0.53)));
    let glass = Material::new_dielectric(1.5);

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

            boxes1.push(Geometry::new_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world: Vec<Geometry> = vec![];
    let mut lights: Vec<Geometry> = vec![];

    world.push(Geometry::new_bvh(boxes1));

    let light_mat = Material::new_diffuse_light(Texture::new_solid(Color::new(7.0, 7.0, 7.0)));
    let light = Geometry::new_quad(
        Point3::new(123.0, 554.0, 147.0),
        Vector3::new(300.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 265.0),
        light_mat,
    );
    world.push(light.clone());
    lights.push(light);

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = &center1 + Vector3::new(30.0, 0.0, 0.0);
    let sphere_material = Material::new_lamb_solid(Color::new(0.7, 0.3, 0.1));
    world.push(Geometry::new_moving_sphere(
        center1,
        center2,
        50.0,
        sphere_material,
    ));

    let glass_sphere = Geometry::new_sphere(Point3::new(260.0, 150.0, 45.0), 50.0, glass.clone());
    world.push(glass_sphere.clone());
    lights.push(glass_sphere);
    world.push(Geometry::new_sphere(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Material::new_metal(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let boundary = Geometry::new_sphere(Point3::new(360.0, 150.0, 145.0), 70.0, glass.clone());
    world.push(boundary.clone());
    world.push(Geometry::new_constant_medium(
        boundary,
        0.2,
        Texture::new_solid(Color::new(0.2, 0.4, 0.9)),
    ));
    let boundary = Geometry::new_sphere(Point3::new(0.0, 0.0, 0.0), 5000.0, glass.clone());
    world.push(Geometry::new_constant_medium(
        boundary,
        0.0001,
        Texture::new_solid(Color::new(1.0, 1.0, 1.0)),
    ));

    let emat = Material::new_lambertian(Texture::new_image("earthmap.jpg"));
    world.push(Geometry::new_sphere(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    ));
    let pertext = Texture::new_noise(0.1);
    world.push(Geometry::new_sphere(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Material::new_lambertian(pertext),
    ));

    let mut boxes2: Vec<Geometry> = vec![];
    let white = Material::new_lamb_solid(Color::new(0.73, 0.73, 0.73));

    for _ in 0..1000 {
        boxes2.push(Geometry::new_sphere(
            Point3::random(0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }

    world.push(Geometry::new_translate(
        Geometry::new_rotate_y(Geometry::new_bvh(boxes2), 15.0),
        Vector3::new(-100.0, 270.0, 395.0),
    ));

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

    Scene::new(world, lights, cam)
}

#[allow(dead_code)]
fn earth() -> Scene {
    let earth_texture = Texture::new_image("earthmap.jpg");
    let earth_surface = Material::new_lambertian(earth_texture);
    let globe = Geometry::new_sphere(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    let light_mat = Material::new_diffuse_light(Texture::new_solid(Color::new(10.0, 10.0, 10.0)));
    let light = Geometry::new_quad(
        Point3::new(-1.0, 5.0, -1.0),
        Vector3::new(2.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 2.0),
        light_mat,
    );

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

    Scene::new(vec![globe], vec![light], cam)
}

fn main() {
    let scene = earth();
    println!("{}", serde_json::to_string(&scene).unwrap());
}
