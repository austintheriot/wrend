use super::{Ray, HitResult};

/// Any object can test whether the ray has hit it
/// t_min and t_max represent the range along a ray
/// where we count a hit "valid"
pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult;
}