mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::{
    camera::Camera,
    hittable_list::HittableList,
    sphere::Sphere,
    vec3::Point3,
};

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Point3::new_with(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Point3::new_with(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();
    camera.render(&world);
}
