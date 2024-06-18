use crate::{
    hittable::{Hittable, HitRecord},
    material::Material,
    ray::Ray,
    utils::Interval,
    vec3::Point3,
};
use std::ops::Range;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center: center.clone(),
            radius: radius.max(0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hits(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let oc = &self.center - &ray.origin();
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
        let outward_normal = &(&p - &self.center) / self.radius;
        let mut rec = HitRecord::new(p, outward_normal, t, self.material.as_ref());
        rec.make_normal_face_ray(ray);
        Some(rec)
    }
}
