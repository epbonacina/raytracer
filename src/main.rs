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

use crate::{
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{LambertianMaterial, Material, Metal},
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();

    let material_of_the_ground = LambertianMaterial::new(&Color::new_with(0.8, 0.8, 0.0));
    let center_ball_material = LambertianMaterial::new(&Color::new_with(0.1, 0.2, 0.5));
    let right_ball_material = Metal::new(&Color::new_with(0.8, 0.6, 0.2), 0.3);
    let left_ball_material = Dielectric::new(1.50);
    let left_bubble_material = Dielectric::new(1.0/1.50);

    world.add(Box::new(Sphere::new(
        &Point3::new_with(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_of_the_ground),
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new_with(0.0, 0.0, -1.2),
        0.5,
        Box::new(center_ball_material),
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new_with(-1.0, 0.0, -1.0),
        0.5,
        Box::new(left_ball_material),
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new_with(-1.0, 0.0, -1.0),
        0.4,
        Box::new(left_bubble_material),
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new_with(1.0, 0.0, -1.0),
        0.5,
        Box::new(right_ball_material),
    )));

    let mut camera = Camera::new();
    camera.lookfrom = Point3::new_with(-2.0, 2.0, 1.0);
    camera.lookat = Point3::new_with(0.0, 0.0, -1.0);
    camera.vup = Vec3::new_with(0.0, 1.0, 0.0);
    camera.vfov = 20.0;
    camera.render(&world);
}
