use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
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
    fn hits(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
