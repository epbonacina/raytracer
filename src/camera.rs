use crate::{
    color::Color,
    hittable::Hittable,
    ray::Ray,
    utils,
    vec3::{Point3, Vec3},
};
use rand::Rng;

pub const MAX_COLOR: u8 = 255;

pub struct Camera {
    aspect_ratio: f64,
    image_width: u16,
    image_height: u16,
    center: Point3,
    samples_per_pixel: u16,
    pixel_samples_scale: f64,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let image_width = 400;
        let aspect_ratio = 16.0 / 9.0;
        let image_height = (image_width as f64 / aspect_ratio) as u16;
        let image_height = std::cmp::max(image_height, 1);

        let center = Point3::new();
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new_with(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new_with(0.0, -viewport_height, 0.0);

        let samples_per_pixel = 100;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let pixel_delta_u = &viewport_u / image_width as f64;
        let pixel_delta_v = &viewport_v / image_height as f64;

        let viewport_upper_left = &(&(&center - &Vec3::new_with(0.0, 0.0, focal_length))
            - &(&viewport_u / 2.0))
            - &(&viewport_v / 2.0);

        let pixel00_loc = &viewport_upper_left + &(0.5 * &(&pixel_delta_u + &pixel_delta_v));

        Camera {
            image_width,
            image_height,
            aspect_ratio,
            center,
            samples_per_pixel,
            pixel_samples_scale,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            log(&format!(
                "Iteration {} (out of {}) [{:.2}%]\r",
                j,
                self.image_height,
                (j as f64 / self.image_height as f64) * 100.0
            ));
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, world);
                }
                let pixel_color = self.pixel_samples_scale * &pixel_color;
                pixel_color.print();
            }
        }
    }

    fn get_ray(&self, i: u16, j: u16) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = &(&self.pixel00_loc + &((i as f64 + offset.x()) * &self.pixel_delta_u))
            + &((j as f64 + offset.y()) * &self.pixel_delta_v);

        let ray_origin = self.center.clone();
        let ray_direction = &pixel_sample - &ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        let num1 = rng.gen_range(-0.5..0.5);
        let num2 = rng.gen_range(-0.5..0.5);
        Vec3::new_with(num1, num2, 0.0)
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color {
        match world.hits(ray, 0.0..utils::INFINITY) {
            Some(hit_record) => 0.5 * &(&hit_record.normal + &Color::new_with(1.0, 1.0, 1.0)),
            None => {
                let unit_direction = ray.direction().unit_vector();
                let a = 0.5 * (unit_direction.y() + 1.0);
                &((1.0 - a) * &Color::new_with(1.0, 1.0, 1.0))
                    + &(a * &Color::new_with(0.5, 0.7, 1.0))
            }
        }
    }
}

fn log(text: &str) {
    eprint!("{}", text);
}
