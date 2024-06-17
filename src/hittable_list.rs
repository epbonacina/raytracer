use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hits(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut temp_hit_record = None;
        for object in self.objects.iter() {
            if let Some(hit_record) = object.hits(ray, ray_tmin, closest_so_far) {
                closest_so_far = hit_record.t;
                temp_hit_record = Some(hit_record);
            }
        }
        temp_hit_record
    }
}
