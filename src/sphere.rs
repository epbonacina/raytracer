use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Sphere {
        Sphere {
            center: center.clone(),
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hits(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = &self.center - &ray.origin();
        let a = &ray.direction().len_squared();
        let h = ray.direction().dot(&oc);
        let c = &oc.len_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            let root = (h+sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = &(&p - &self.center) / self.radius;
        let mut rec = HitRecord::new(p, outward_normal, t);
        rec.make_normal_face_ray(ray);
        Some(rec)
    }
}
