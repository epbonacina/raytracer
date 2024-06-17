mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

const IMAGE_WIDTH: u16 = 400;
const MAX_COLOR: u16 = 255;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn log(text: &str) {
    eprint!("{}", text);
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    match world.hits(ray, 0.0..utils::INFINITY) {
        Some(hit_record) => 0.5 * &(&hit_record.normal + &Color::new_with(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = ray.direction().unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.0);
            &((1.0 - a) * &Color::new_with(1.0, 1.0, 1.0)) + &(a * &Color::new_with(0.5, 0.7, 1.0))
        }
    }
}

fn main() {
    let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
    let image_height = std::cmp::max(image_height, 1);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Point3::new_with(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Point3::new_with(0.0, -100.5, -1.0), 100.0)));

    let focal_length = 1.0;
    let camera_center = Point3::new();
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f64 / image_height as f64);

    let viewport_u = Vec3::new_with(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new_with(0.0, -viewport_height, 0.0);

    let pixel_delta_u = &viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    let viewport_upper_left = &(&(&camera_center - &Vec3::new_with(0.0, 0.0, focal_length))
        - &(&viewport_u / 2.0))
        - &(&viewport_v / 2.0);

    let pixel00_loc = &viewport_upper_left + &(0.5 * &(&pixel_delta_u + &pixel_delta_v));

    println!("P3");
    println!("{IMAGE_WIDTH} {image_height}");
    println!("{MAX_COLOR}");

    for j in 0..image_height {
        log(&format!(
            "Iteration {j} (out of {image_height}) [{:.2}%]\r",
            (j as f64 / image_height as f64) * 100.0
        ));
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                &pixel00_loc + &(&(i as f64 * &pixel_delta_u) + &(j as f64 * &pixel_delta_v));

            let ray_direction = &pixel_center - &camera_center;
            let ray = Ray::new(camera_center.clone(), ray_direction);

            let pixel_color = ray_color(&ray, &world);
            pixel_color.print();
        }
    }
}
