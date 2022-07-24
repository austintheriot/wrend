//! This crate is a mirror of much of the GLSL code already written
//! and is intended to interop well with the GPU side of things.

use super::{Hit, HitResult, HitResultData, Material, Ray};
use wrend::Vec3;

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
    pub uuid: i32,
}

impl Hit for Sphere {
    fn hit(&self, ray: &super::ray::Ray, t_min: f64, t_max: f64) -> HitResult {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        // no hit
        if discriminant < 0. {
            return HitResult::NoHit;
        }

        // there is a hit, but it may not be within the acceptable range:
        // find the nearest root that lies in the acceptable range.
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-half_b - sqrt_discriminant) / a;

        // t is out of range, so count it as a no hit
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return HitResult::NoHit;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (&hit_point - &self.center) / self.radius;

        let hit_result_data = HitResultData::builder()
            .t(root)
            .hit_point(hit_point)
            .front_face_and_normal(ray, &outward_normal)
            .uuid(self.uuid)
            .build();

        HitResult::Hit {
            data: hit_result_data,
        }
    }
}

pub fn set_sphere_uuids(spheres: &mut Vec<Sphere>) {
    for (i, sphere) in spheres.iter_mut().enumerate() {
        sphere.uuid = i as i32;
    }
}

pub fn get_center_hit(spheres: &Vec<Sphere>, ray: Ray) -> HitResult {
    let mut prev_hit_result = HitResult::NoHit;
    let mut closest_so_far = f64::INFINITY;

    for sphere in spheres {
        let new_hit_result = sphere.hit(&ray, 0., closest_so_far);

        // this object was a hit (and implicitly was in front of the last)
        if let HitResult::Hit {
            data: ref new_hit_data,
        } = new_hit_result
        {
            closest_so_far = new_hit_data.t;
            prev_hit_result = new_hit_result;
        }
    }

    prev_hit_result
}
