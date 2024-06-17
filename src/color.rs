use crate::{camera::MAX_COLOR, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    pub fn print(&self) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let ru = r * MAX_COLOR as f64;
        let gu = g * MAX_COLOR as f64;
        let bu = b * MAX_COLOR as f64;

        println!("{ru} {gu} {bu}");
    }
}
