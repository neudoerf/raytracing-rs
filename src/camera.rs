use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    color::Color, hittable::Hittable, interval::Interval, material::ScatterRecord, pdf,
    point3::Point3, ray::Ray, vector3::Vector3,
};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: usize,
    max_depth: u32,

    background: Color,

    defocus_angle: f64,

    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
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
        let image_height = (image_width as f64 / aspect_ratio).floor().max(1.0) as u32;
        let center = look_from.clone();

        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (&look_from - &look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let viewport_u = viewport_width * &u;
        let viewport_v = viewport_height * -&v;

        let pixel_delta_u = &viewport_u / image_width as f64;
        let pixel_delta_v = &viewport_v / image_height as f64;

        let viewport_upper_left =
            &center - (focus_dist * &w) - &viewport_u / 2.0 - &viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = &u * defocus_radius;
        let defocus_disk_v = &v * defocus_radius;

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            background,
            defocus_angle,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &Hittable, lights: &Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        let image: Vec<Vec<_>> = (0..self.image_height)
            .into_par_iter()
            .progress_count(self.image_height.into())
            .map(|j| {
                // eprint!("\rScanlines remaining: {}   ", self.image_height - j);
                (0..self.image_width)
                    .into_par_iter()
                    .map(|i| {
                        (0..self.samples_per_pixel)
                            .into_par_iter()
                            .map(|_| {
                                let r = self.get_ray(i, j);
                                self.ray_color(&r, self.max_depth, world, lights)
                            })
                            .reduce(|| Color::new(0.0, 0.0, 0.0), |a, b| a + b)
                    })
                    .collect()
            })
            .collect();
        image.iter().for_each(|row| {
            row.iter().for_each(|c| {
                println!("{}", c.write(self.samples_per_pixel));
            })
        })
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &Hittable, lights: &Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        world
            .hit(r, Interval::new(0.001, f64::MAX))
            .and_then(|rec| {
                let color_from_emission = rec.material.emitted(&r, &rec, rec.u, rec.v, &rec.p);
                rec.material
                    .scatter(r, &rec)
                    .and_then(|srec| match srec {
                        ScatterRecord::Ray(attenuation, scatter) => {
                            Some(attenuation * self.ray_color(&scatter, depth - 1, world, lights))
                        }
                        ScatterRecord::Pdf(attenuation, rec_pdf) => {
                            let light_pdf = pdf::Hittable::new(&lights, rec.p.clone());
                            let p = pdf::Mixture::new(&light_pdf, &rec_pdf);
                            let scattered = Ray::new(rec.p.clone(), p.generate(), r.time);
                            let pdf_val = p.value(&scattered.dir);

                            let scattering_pdf = rec.material.scattering_pdf(r, &rec, &scattered);
                            let sample_color = self.ray_color(&scattered, depth - 1, world, lights);

                            let color_from_scatter =
                                (attenuation * scattering_pdf * sample_color) / pdf_val;

                            Some(&color_from_emission + color_from_scatter)
                        }
                    })
                    .or(Some(color_from_emission))
            })
            .unwrap_or(self.background.clone())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let mut rng = rand::thread_rng();
        let pixel_center =
            &self.pixel00_loc + (i as f64 * &self.pixel_delta_u) + (j as f64 * &self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let orig = if self.defocus_angle <= 0.0 {
            self.center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let dir = pixel_sample - &orig;
        let time = rng.gen();

        Ray::new(orig, dir, time)
    }

    fn pixel_sample_square(&self) -> Vector3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 * rng.gen::<f64>();
        let py = -0.5 * rng.gen::<f64>();
        px * &self.pixel_delta_u + py * &self.pixel_delta_v
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vector3::random_in_unit_disk();
        &self.center + p.x * &self.defocus_disk_u + p.y * &self.defocus_disk_v
    }
}
