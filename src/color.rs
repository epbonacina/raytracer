use crate::{camera::MAX_COLOR, utils::Interval, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    pub fn print(&self) {
        let r = Color::linear_to_gamma(self.x());
        let g = Color::linear_to_gamma(self.y());
        let b = Color::linear_to_gamma(self.z());

        let intensity = 0.0..0.999;
        let ru = MAX_COLOR as f64 * intensity.make_fit(r);
        let gu = MAX_COLOR as f64 * intensity.make_fit(g);
        let bu = MAX_COLOR as f64 * intensity.make_fit(b);

        println!("{ru} {gu} {bu}");
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        0.0
    }
}
