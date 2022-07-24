use super::Ray;
use wrend::{Point, Vec3};

#[derive(Debug, Default, Clone)]
pub struct HitResultData {
    pub hit_point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub uuid: i32,
}

impl HitResultData {
    pub fn builder() -> HitResultDataBuilder {
        HitResultDataBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct HitResultDataBuilder {
    hit_point: Point,
    t: f64,
    normal: Vec3,
    front_face: bool,
    uuid: i32,
}

impl HitResultDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hit_point(mut self, hit_point: Point) -> Self {
        self.hit_point = hit_point;
        self
    }

    pub fn t(mut self, t: f64) -> Self {
        self.t = t;
        self
    }

    pub fn uuid(mut self, uuid: i32) -> Self {
        self.uuid = uuid;
        self
    }

    pub fn front_face_and_normal(mut self, r: &Ray, outward_normal: &Vec3) -> Self {
        self.front_face = Vec3::dot(&r.direction, outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
        self
    }

    pub fn build(self) -> HitResultData {
        HitResultData {
            hit_point: self.hit_point,
            normal: self.normal,
            t: self.t,
            front_face: self.front_face,
            uuid: self.uuid,
        }
    }
}
