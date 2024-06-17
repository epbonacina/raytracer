use std::ops::Range;

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = std::f64::INFINITY;
pub const EMPTY_INTERVAL: Range<f64> = 0.0..0.0;
pub const UNIVERSE_INTERVAL: Range<f64> = std::f64::NEG_INFINITY..std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub trait Interval {
    fn surrounds(&self, value: f64) -> bool;
    fn make_fit(&self, value: f64) -> f64;
}

impl Interval for Range<f64> {
    fn surrounds(&self, value: f64) -> bool {
        self.start < value && self.end > value
    }

    fn make_fit(&self, value: f64) -> f64 {
        let value = value.max(self.start);
        value.min(self.end)
    }
}
