use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    utils::Interval,
    vec3::Point3,
};
use std::ops::Range;

pub struct Sphere {
    center_start: Point3,
    center_end: Point3,
    radius: f64,
    start_time: f64,
    end_time: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(
        center_start: &Point3,
        center_end: &Point3,
        radius: f64,
        time0: f64,
        time1: f64,
        material: Box<dyn Material>,
    ) -> Sphere {
        Sphere {
            center_start: center_start.clone(),
            center_end: center_end.clone(),
            radius: radius.max(0.0),
            start_time: time0,
            end_time: time1,
            material,
        }
    }

    fn center(&mut self, current_time: f64) -> Point3 {
        if self.start_time < self.end_time {
            let t = (current_time - self.start_time) / (self.end_time - self.start_time);
            let c = &self.center_end - &self.center_start;
            &self.center_start + &(t * &c)
        } else {
            self.center_start.clone()
        }
    }
}

impl Hittable for Sphere {
    fn hits(&mut self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let oc = &self.center(ray.time()) - ray.origin();
        let a = &ray.direction().len_squared();
        let h = ray.direction().dot(&oc);
        let c = &oc.len_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = &(&p - &self.center(ray.time())) / self.radius;
        let mut rec = HitRecord::new(p, outward_normal, t, self.material.as_ref());
        rec.make_normal_face_ray(ray);
        Some(rec)
    }
}
