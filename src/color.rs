use crate::{camera::MAX_COLOR, utils::Interval, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    pub fn print(&self) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let intensity = 0.0..0.999;
        let ru = MAX_COLOR as f64 * intensity.make_fit(r);
        let gu = MAX_COLOR as f64 * intensity.make_fit(g);
        let bu = MAX_COLOR as f64 * intensity.make_fit(b);

        println!("{ru} {gu} {bu}");
    }
}
