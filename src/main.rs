mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use material::Dielectric;
use rand::Rng;

use crate::{
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{LambertianMaterial, Metal},
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();

    let material_of_the_ground = LambertianMaterial::new(&Color::new_with(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        &Point3::new_with(0.0, -1000.0, 0.0),
        &Point3::new_with(0.0, -1000.0, 0.0),
        1000.0,
        0.0,
        0.0,
        Box::new(material_of_the_ground),
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let chosen_material = rng.gen_range(0.0..1.0);
            let center_start = Point3::new_with(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );
            if (&center_start - &Point3::new_with(4.0, 0.2, 0.0)).len() > 0.9 {
                if chosen_material < 0.8 {
                    let albedo = &Color::random() * &Color::random();
                    let sphere_material = Box::new(LambertianMaterial::new(&albedo));
                    let center_end = &center_start + &Vec3::new_with(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Box::new(Sphere::new(
                        &center_start,
                        &center_end,
                        0.2,
                        0.0,
                        1.0,
                        sphere_material,
                    )));
                } else if chosen_material < 0.95 {
                    let albedo = &Color::random_within(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Box::new(Metal::new(&albedo, fuzz));
                    world.add(Box::new(Sphere::new(
                        &center_start,
                        &center_start,
                        0.2,
                        0.0,
                        0.0,
                        sphere_material,
                    )));
                } else {
                    let sphere_material = Box::new(Dielectric::new(1.50));
                    world.add(Box::new(Sphere::new(
                        &center_start,
                        &center_start,
                        0.2,
                        0.0,
                        0.0,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let left_ball_material = LambertianMaterial::new(&Color::new_with(0.1, 0.2, 0.5));
    let right_ball_material = Metal::new(&Color::new_with(0.8, 0.6, 0.2), 0.0);
    let middle_ball_material = Dielectric::new(1.50);
    let middle_ball_component_material = Dielectric::new(1.0 / 1.50);

    world.add(Box::new(Sphere::new(
        &Point3::new_with(0.0, 1.0, 0.0),
        &Point3::new_with(0.0, 1.0, 0.0),
        1.0,
        0.0,
        0.0,
        Box::new(left_ball_material),
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new_with(2.0, 1.0, 0.0),
        &Point3::new_with(2.0, 1.0, 0.0),
        1.0,
        0.0,
        0.0,
        Box::new(middle_ball_material),
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new_with(2.0, 1.0, 0.0),
        &Point3::new_with(2.0, 1.0, 0.0),
        0.9,
        0.0,
        0.0,
        Box::new(middle_ball_component_material),
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new_with(4.0, 1.0, 0.0),
        &Point3::new_with(4.0, 1.0, 0.0),
        1.0,
        0.0,
        0.0,
        Box::new(right_ball_material),
    )));

    let mut camera = Camera::new();
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_bounces = 10;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new_with(13.0, 2.0, 3.0);
    camera.lookat = Point3::new_with(0.0, 0.0, 0.0);
    camera.vup = Vec3::new_with(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    camera.start_time = 0.0;
    camera.end_time = 1.0;
    camera.render(&mut world);
}
