use std::ops::Range;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point3, normal: Vec3, t: f64, material: &dyn Material) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            material,
            front_face: false,
        }
    }

    pub fn make_normal_face_ray(&mut self, ray: &Ray) {
        let they_already_face_each_other = ray.direction().dot(&self.normal) < 0.0;
        if they_already_face_each_other {
            self.front_face = true;
        } else {
            self.front_face = false;
            self.normal = -&self.normal;
        }
    }
}

pub trait Hittable {
    fn hits(&mut self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord>;
}
