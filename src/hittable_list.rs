use std::ops::Range;
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
    fn hits(&mut self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.end;
        let mut temp_hit_record = None;
        for object in self.objects.iter_mut() {
            if let Some(hit_record) = object.hits(ray, ray_t.start..closest_so_far) {
                closest_so_far = hit_record.t;
                temp_hit_record = Some(hit_record);
            }
        }
        temp_hit_record
    }
}
