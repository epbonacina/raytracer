use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    #[allow(unused_variables)]
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
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
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = &rec.normal + &Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal.clone();
        }

        let scattered = Ray::new(rec.p.clone(), scatter_direction);
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
        let scattered = Ray::new(rec.p.clone(), reflected);
        let attenuation = self.albedo.clone();
        Some((attenuation, scattered))
    }
}
