use super::{Hit, Ray, HitResult};

#[derive(Default)]
pub struct HittableList {
    pub list: Vec<Box<dyn Hit>>,
}

/// creates a list of hittable objects without having to write `Box::new()`
/// around each item that is included in the list.
#[macro_export]
macro_rules! hittable_list {
  ($($hittable: expr),*) => {{
       let mut list: Vec<Box<dyn Hit>> = Vec::new();
       $( list.push(Box::new($hittable)); )*
       HittableList { list }
  }}
}

impl Hit for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult {
        let mut prev_hit_result = HitResult::NoHit;

        for hittable in &self.list {
            let new_hit_result = hittable.hit(ray, t_min, t_max);

            // this object was a hit
            if let HitResult::Hit { data: new_hit_data } = &new_hit_result {
                // replace saved hit result if previous was no-hit or was behind this new one
                match &prev_hit_result {
                    HitResult::NoHit => prev_hit_result = new_hit_result,
                    HitResult::Hit {
                        data: prev_hit_data,
                    } => {
                        if new_hit_data.hit_point.z() > prev_hit_data.hit_point.z() {
                            prev_hit_result = new_hit_result
                        }
                    }
                }
            }
        }

        prev_hit_result
    }
}
