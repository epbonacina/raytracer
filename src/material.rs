use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};
use rand::Rng;

use std::{thread, time};

pub trait Material {
    #[allow(unused_variables)]
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    #[allow(unused_variables)]
    fn emit(&self, ray_in: &Ray, rec: &HitRecord) -> Color {
        Color::new()
    }
}

pub struct LambertianMaterial {
    albedo: Color,
}

impl LambertianMaterial {
    pub fn new(albedo: &Color) -> LambertianMaterial {
        LambertianMaterial {
            albedo: albedo.clone(),
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = &rec.normal + &Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal.clone();
        }

        let scattered = Ray::new(rec.p.clone(), scatter_direction, ray_in.time());
        let attenuation = self.albedo.clone();
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo.clone(),
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().reflect(&rec.normal);
        let reflected = &reflected.unit_vector() + &(self.fuzz * &Vec3::random_unit_vector());
        let scattered = Ray::new(rec.p.clone(), reflected, ray_in.time());
        let attenuation = self.albedo.clone();
        Some((attenuation, scattered))
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0.powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().unit_vector();

        let cos_theta = -&unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0.0..1.0);
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_number
        {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, ri)
        };

        let scattered = Ray::new(rec.p.clone(), direction, ray_in.time());
        let attenuation = Color::new_with(1.0, 1.0, 1.0);
        Some((attenuation, scattered))
    }
}

pub struct LightDiffuser {
    color: Color,
}

impl LightDiffuser {
    pub fn new(color: &Color) -> LightDiffuser {
        LightDiffuser {
            color: color.clone(),
        }
    }
}

impl Material for LightDiffuser {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emit(&self, _ray_in: &Ray, _rec: &HitRecord) -> Color {
        self.color.clone()
    }
}

pub struct Smoke {
    color: Color,
    density: f64,
}

impl Smoke {
    pub fn new(color: &Color, density: f64) -> Smoke {
        Smoke {
            color: color.clone(),
            density,
        }
    }
}

impl Material for Smoke {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0.1..0.8);

        if random_number > self.density {
            let mut scatter_direction = &rec.normal + &Vec3::random_unit_vector();

            if scatter_direction.is_near_zero() {
                scatter_direction = rec.normal.clone();
            }

            let scattered = Ray::new(rec.p.clone(), scatter_direction, ray_in.time());
            let attenuation = self.color.clone();
            Some((attenuation, scattered))
        } else {
            let scattered = Ray::new(rec.p.clone(), ray_in.direction().clone(), ray_in.time());
            let attenuation = Color::new_with(1.0, 1.0, 1.0);
            Some((attenuation, scattered))
        }
    }
}
