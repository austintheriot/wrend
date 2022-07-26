use std::f64::consts::PI;

use wrend::{degrees_to_radians, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Pipeline {
    width: u32,
    height: u32,
    aspect_ratio: f64,
    focal_length: f64,
    camera_origin: Vec3,
    pitch: f64,
    yaw: f64,
    camera_front: Vec3,
    vup: Vec3,
    /// stored in radians
    camera_field_of_view: f64,
    camera_h: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    aperture: f64,
    lens_radius: f64,
    focus_distance: f64,
    viewport_height: f64,
    viewport_width: f64,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    look_at: Vec3,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            aperture: Self::DEFAULT_APERTURE,
            focus_distance: Self::DEFAULT_FOCUS_DISTANCE,
            lens_radius: Self::DEFAULT_LENS_RADIUS,
            camera_field_of_view: Self::DEFAULT_CAMERA_FIELD_OF_VIEW,
            focal_length: Self::DEFAULT_FOCAL_LENGTH,
            pitch: Self::DEFAULT_PITCH,
            yaw: Self::DEFAULT_YAW,
            vup: Self::DEFAULT_VUP,
            camera_origin: Self::DEFAULT_CAMERA_ORIGIN,
            aspect_ratio: Default::default(),
            camera_front: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            viewport_height: Default::default(),
            viewport_width: Default::default(),
            horizontal: Default::default(),
            vertical: Default::default(),
            lower_left_corner: Default::default(),
            camera_h: Default::default(),
            look_at: Default::default(),
        }
    }
}

impl Pipeline {
    const DEFAULT_WIDTH: u32 = 1;
    const DEFAULT_HEIGHT: u32 = 1;
    const DEFAULT_APERTURE: f64 = 0.;
    const DEFAULT_FOCUS_DISTANCE: f64 = 0.75;
    const DEFAULT_LENS_RADIUS: f64 = Self::DEFAULT_APERTURE / 2.0;
    const DEFAULT_CAMERA_FIELD_OF_VIEW: f64 = PI / 3.;
    const DEFAULT_FOCAL_LENGTH: f64 = 1.0;
    const DEFAULT_PITCH: f64 = 0.0;
    // look down the z axis by default
    const DEFAULT_YAW: f64 = -90.0;
    const DEFAULT_VUP: Vec3 = Vec3::new(0., 1., 0.);
    const DEFAULT_CAMERA_ORIGIN: Vec3 = Vec3::new(0., 0., 1.);

    pub fn new(width: u32, height: u32) -> Self {
        let mut new_pipeline = Self::default();
        new_pipeline.set_width_and_height(width, height);
        new_pipeline.update_pipeline();
        new_pipeline
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn camera_front(&self) -> Vec3 {
        self.camera_front
    }

    pub fn vup(&self) -> Vec3 {
        self.vup
    }

    pub fn camera_field_of_view(&self) -> f64 {
        self.camera_field_of_view
    }

    pub fn camera_origin(&self) -> Vec3 {
        self.camera_origin
    }

    pub fn lower_left_corner(&self) -> Vec3 {
        self.lower_left_corner
    }

    pub fn horizontal(&self) -> Vec3 {
        self.horizontal
    }

    pub fn vertical(&self) -> Vec3 {
        self.vertical
    }

    pub fn aperture(&self) -> f64 {
        self.aperture
    }

    pub fn lens_radius(&self) -> f64 {
        self.lens_radius
    }

    pub fn u(&self) -> Vec3 {
        self.u
    }

    pub fn v(&self) -> Vec3 {
        self.v
    }

    pub fn pitch(&self) -> f64 {
        self.pitch
    }

    pub fn yaw(&self) -> f64 {
        self.yaw
    }

    pub fn set_width_and_height(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.update_pipeline();
    }

    pub fn set_camera_field_of_view(&mut self, camera_field_of_view: f64) {
        self.camera_field_of_view = camera_field_of_view.clamp(0.0001, PI * 0.75);
        self.update_pipeline();
    }

    pub fn set_pitch_and_yaw(&mut self, pitch: f64, yaw: f64) {
        self.pitch = f64::clamp(pitch, -89., 89.);
        self.yaw = yaw;
        self.update_pipeline();
    }

    pub fn set_camera_origin(&mut self, camera_origin: Vec3) {
        self.camera_origin = camera_origin;
        self.update_pipeline();
    }

    pub fn set_focus_distance(&mut self, focus_distance: f64) {
        self.focus_distance = focus_distance;
        self.update_pipeline();
    }

    fn update_pipeline(&mut self) {
        self.aspect_ratio = (self.width as f64) / (self.height as f64);
        self.camera_h = (self.camera_field_of_view / 2.).tan();
        self.camera_front = Vec3::new(
            f64::cos(degrees_to_radians(self.yaw)) * f64::cos(degrees_to_radians(self.pitch)),
            f64::sin(degrees_to_radians(self.pitch)),
            f64::sin(degrees_to_radians(self.yaw)) * f64::cos(degrees_to_radians(self.pitch)),
        );
        self.look_at = self.camera_origin + self.camera_front;
        self.w = Vec3::normalize(self.camera_origin - self.look_at);
        self.u = Vec3::normalize(Vec3::cross(self.vup, self.w));
        self.v = Vec3::cross(self.w, self.u);
        self.viewport_height = 2. * self.camera_h;
        self.viewport_width = self.viewport_height * self.aspect_ratio;
        self.horizontal = self.focus_distance * self.viewport_width * self.u;
        self.vertical = self.focus_distance * self.viewport_height * self.v;
        self.lower_left_corner = self.camera_origin
            - self.horizontal / 2.
            - self.vertical / 2.
            - self.focus_distance * self.w;
    }
}
