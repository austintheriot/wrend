use js_sys::Math;
use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180.
}

pub fn random_with_range(min: f64, max: f64) -> f64 {
    min + (max - min) * Math::random()
}
