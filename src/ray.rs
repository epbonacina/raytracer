use crate::vec3::{Vec3, Point3};

pub struct Ray {
    direction: Vec3,
    origin: Point3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            direction,
            origin,
            time,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
    
    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(t * &(self.direction))
    }
}
