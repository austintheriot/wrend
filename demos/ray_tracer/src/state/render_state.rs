use std::ops::Add;

use super::camera::Camera;
use crate::{
    controls::{keydown_key::KeydownKey, KeydownState},
    objects::{self, HitResult, Material, MaterialType, Ray, Sphere},
    utils,
};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlTexture, window};
use wrend::Vec3;

pub type RenderStateCount = u32;

pub const MOVEMENT_SPEED: f64 = 0.001;

/// The interval that must elapse before the canvas size is synced with the window size
/// stored in `ms`
pub const RESIZE_UPDATE_DEBOUNCE_INTERVAL: f64 = 500.0;

/// This must match the amount in the shader itself
/// @todo programmatically set the value in the shader before compiling
/// Or use the `MAX_FRAGMENT_UNIFORM_VECTORS` provided by the WebGL context
pub const MAX_NUM_SPHERES: u8 = 15;

/// So high that it's unlikely to be a real id of an object in the shader
/// Since GLSL has no concept of `None`, it's easier to store this as a plain number
pub const NO_SELECTED_OBJECT_ID: i32 = 1000;

pub const MAX_FRAME_AVERAGES: u32 = 100_000;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RenderState {
    camera: Camera,

    // RENDER STATE
    samples_per_pixel: u32,
    /// number of ray reflections to calculate
    max_depth: u32,
    sphere_list: Vec<Sphere>,
    /// is the modal up that asks the user to enable first-person viewing mode?
    is_paused: bool,
    /// Whether the browser should save a screenshot of the canvas
    should_save_image: bool,
    /// Used for averaging previous frames together
    /// and also for determine which framebuffer to render to
    render_count: u32,
    /// The weight of the last frame compared to the each frame before.
    last_frame_weight: f32,
    /// Limiting the counted renders allows creating a sliding average of frames
    max_render_count: u32,
    /// Used for calculating time delta in animation loop
    prev_now: f64,
    /// this is necessary after the user resizes their viewport
    window_size_out_of_sync: bool,
    prev_resize_sync_time: f64,

    // MOVEMENT
    keydown_state: KeydownState,
    look_sensitivity: f64,

    // DEBUGGING
    debugging_enabled: i32,
    cursor_point: Vec3,
    selected_object: i32,
}

impl Default for RenderState {
    fn default() -> Self {
        // just uses default 1x1px size at first:
        // this is updated at initialization time
        let pipeline = Camera::default();
        let samples_per_pixel = 1;
        let max_depth = 8;
        let should_save_image = false;
        let render_count = 0;
        let last_frame_weight = 1.;
        let max_render_count = 100_000;
        let prev_now = 0.;
        // let width / height become synced on the first render
        let window_size_out_of_sync = true;
        let prev_resize_sync_time = 0.0;

        let is_paused = true;

        let look_sensitivity = 0.1;
        let keydown_state = KeydownState::default();

        let enable_debugging = 0;
        let cursor_point = Vec3::new(0., 0., 0.);
        let selected_object = NO_SELECTED_OBJECT_ID;

        let sphere_list = create_default_sphere_list();

        RenderState {
            camera: pipeline,

            samples_per_pixel,
            max_depth,

            is_paused,
            should_save_image,
            render_count,
            last_frame_weight,
            max_render_count,
            prev_now,
            window_size_out_of_sync,
            prev_resize_sync_time,

            keydown_state,
            look_sensitivity,

            debugging_enabled: enable_debugging,
            cursor_point,
            selected_object,

            sphere_list,
        }
    }
}

impl RenderState {
    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    pub fn sphere_list(&self) -> &[Sphere] {
        &self.sphere_list
    }

    pub fn max_depth(&self) -> u32 {
        self.max_depth
    }

    pub fn selected_object(&self) -> i32 {
        self.selected_object
    }

    pub fn cursor_point(&self) -> Vec3 {
        self.cursor_point
    }

    pub fn prev_now(&self) -> f64 {
        self.prev_now
    }

    pub fn debugging_enabled(&self) -> i32 {
        self.debugging_enabled
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn should_save_image(&self) -> bool {
        self.should_save_image
    }

    pub fn render_count(&self) -> u32 {
        self.render_count
    }

    pub fn prev_resize_sync_time(&self) -> f64 {
        self.prev_resize_sync_time
    }

    pub fn window_size_out_of_sync(&self) -> bool {
        self.window_size_out_of_sync
    }

    pub fn look_sensitivity(&self) -> f64 {
        self.look_sensitivity
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn keydown_state(&self) -> &KeydownState {
        &self.keydown_state
    }

    pub fn keydown_state_mut(&mut self) -> &mut KeydownState {
        &mut self.keydown_state
    }

    pub fn reset_render_count(&mut self) -> &mut Self {
        self.render_count = 0;
        self
    }

    pub fn inc_render_count(&mut self) -> &mut Self {
        self.render_count = self.render_count.add(1).min(MAX_FRAME_AVERAGES);
        self
    }

    pub fn set_window_size_out_of_sync(&mut self, window_size_out_of_sync: bool) -> &mut Self {
        self.window_size_out_of_sync = window_size_out_of_sync;
        self
    }

    pub fn set_is_paused(&mut self, is_paused: bool) -> &mut Self {
        self.is_paused = is_paused;
        self
    }

    pub fn set_prev_now(&mut self, prev_now: f64) -> &mut Self {
        self.prev_now = prev_now;
        self
    }

    pub fn set_should_save_image(&mut self, should_save_image: bool) -> &mut Self {
        self.should_save_image = should_save_image;
        self
    }

    /// Synchronizes the window's size with internal state
    /// WebGL, the canvas, and all appropriate textures
    pub fn sync_dimensions(
        &mut self,
        gl: &WebGl2RenderingContext,
        render_textures: &[WebGlTexture],
        canvas: &HtmlCanvasElement,
        now: f64,
    ) -> &mut Self {
        // update state
        self.prev_resize_sync_time = now;
        self.reset_render_count();
        self.set_window_size_out_of_sync(false);

        // sync internal state
        let (width, height) = utils::clamped_screen_dimensions();
        let render_state_pipeline: &mut Camera = self.as_mut();
        render_state_pipeline.set_width_and_height(width, height);

        // sync canvas
        canvas.set_width(width);
        canvas.set_height(height);

        //sync WebGL
        gl.viewport(0, 0, width as i32, height as i32);

        // sync textures that get rendered into
        for texture in render_textures.iter() {
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                width as i32,
                height as i32,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                None,
            )
            .unwrap();
        }

        self
    }

    pub fn update_position(&mut self) -> &mut Self {
        let now = window().unwrap().performance().unwrap().now();
        let dt = now - self.prev_now();
        self.set_prev_now(now);

        if self.keydown_state.no_keys_down() {
            return self;
        }

        let pipeline = self.camera();
        let camera_front = pipeline.camera_front();
        let vup = pipeline.vup();
        let fov = pipeline.camera_field_of_view();
        let camera_origin = pipeline.camera_origin();

        let new_camera_origin = if self.keydown_state[KeydownKey::W] {
            camera_origin + &camera_front * MOVEMENT_SPEED * dt * fov
        } else if self.keydown_state[KeydownKey::A] {
            camera_origin - Vec3::cross(&camera_front, &vup) * MOVEMENT_SPEED * dt * fov
        } else if self.keydown_state[KeydownKey::S] {
            camera_origin - &camera_front * MOVEMENT_SPEED * dt * fov
        } else if self.keydown_state[KeydownKey::D] {
            camera_origin + Vec3::cross(&camera_front, &vup) * MOVEMENT_SPEED * dt * fov
        } else if self.keydown_state[KeydownKey::Space] {
            camera_origin + &vup * MOVEMENT_SPEED * dt * fov
        } else if self.keydown_state[KeydownKey::Shift] {
            camera_origin - &vup * MOVEMENT_SPEED * dt * fov
        } else {
            camera_origin
        };

        if new_camera_origin != camera_origin {
            self.camera_mut().set_camera_origin(new_camera_origin);
            self.reset_render_count();
        }

        self
    }

    /// focus on whatever object is selected by the cursor if there was a collision
    pub fn update_cursor_position_in_world(&mut self) -> &mut Self {
        let pipeline = self.camera();
        let camera_origin = pipeline.camera_origin();
        let lower_left_corner = pipeline.lower_left_corner();
        let horizontal = pipeline.horizontal();
        let vertical = pipeline.vertical();
        let aperture = pipeline.aperture();

        let ray = Ray {
            origin: camera_origin,
            direction: lower_left_corner + horizontal / 2. + vertical / 2. - camera_origin,
        };

        let spheres = &self.sphere_list;

        if let HitResult::Hit { data } = objects::get_center_hit(spheres, ray) {
            let distance = (data.hit_point - camera_origin).length();
            if aperture > 0. {
                // there is no blurring if aperture is zero
                self.camera_mut().set_focus_distance(distance);
            }
            self.cursor_point = data.hit_point;
            self.selected_object = data.uuid;
        } else {
            if aperture > 0. {
                // there is no blurring if aperture is zerp
                self.camera_mut().set_focus_distance(10.0);
            }
            self.cursor_point = Vec3::new(0., 0., 0.);
            self.selected_object = NO_SELECTED_OBJECT_ID;
        }

        self
    }
}

impl AsMut<Camera> for RenderState {
    fn as_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

impl AsRef<Camera> for RenderState {
    fn as_ref(&self) -> &Camera {
        &self.camera
    }
}

impl AsMut<KeydownState> for RenderState {
    fn as_mut(&mut self) -> &mut KeydownState {
        &mut self.keydown_state
    }
}

impl AsRef<KeydownState> for RenderState {
    fn as_ref(&self) -> &KeydownState {
        &self.keydown_state
    }
}

pub fn create_default_sphere_list() -> Vec<Sphere> {
    let mut sphere_list = vec![
        // ground
        Sphere {
            center: Vec3::new(0., -100.5, -1.),
            radius: 100.,
            material: Material {
                material_type: MaterialType::Diffuse,
                albedo: Vec3::new(0.75, 0.6, 0.5),
                fuzz: 0.,
                refraction_index: 0.,
            },
            uuid: 0,
        },
        // center (blue)
        Sphere {
            center: Vec3::new(0., 0., -1.),
            radius: 0.5,
            material: Material {
                material_type: MaterialType::Diffuse,
                albedo: Vec3::new(0.3, 0.3, 0.4),
                fuzz: 0.,
                refraction_index: 0.,
            },
            uuid: 0,
        },
        // left
        Sphere {
            center: Vec3::new(-1.1, 0., -1.),
            radius: 0.5,
            material: Material {
                material_type: MaterialType::Metal,
                albedo: Vec3::new(1.0, 1.0, 1.0),
                fuzz: 0.,
                refraction_index: 0.,
            },
            uuid: 0,
        },
        // right
        Sphere {
            center: Vec3::new(1.1, 0., -1.),
            radius: 0.5,
            material: Material {
                material_type: MaterialType::Glass,
                albedo: Vec3::new(1.0, 1.0, 1.0),
                fuzz: 0.,
                refraction_index: 1.5,
            },
            uuid: 0,
        },
        // back left (shiny)
        Sphere {
            center: Vec3::new(-0.5, -0.35, -0.55),
            radius: -0.15,
            material: Material {
                material_type: MaterialType::Metal,
                albedo: Vec3::new(1.0, 1.0, 1.0),
                fuzz: 0.,
                refraction_index: 0.,
            },
            uuid: 0,
        },
        // front left (fuzzy)
        Sphere {
            center: Vec3::new(-0.75, -0.4, -0.35),
            radius: -0.1,
            material: Material {
                material_type: MaterialType::Metal,
                albedo: Vec3::new(1.0, 1.0, 1.0),
                fuzz: 0.,
                refraction_index: 0.,
            },
            uuid: 0,
        },
        // behind
        Sphere {
            center: Vec3::new(0., 1.2, 4.),
            radius: 2.,
            material: Material {
                material_type: MaterialType::Diffuse,
                albedo: Vec3::new(1.0, 0.8, 0.8),
                fuzz: 0.,
                refraction_index: 0.,
            },
            uuid: 0,
        },
        // distant (moon)
        Sphere {
            center: Vec3::new(150., 20., -500.),
            radius: 100.,
            material: Material {
                material_type: MaterialType::Diffuse,
                albedo: Vec3::new(0.95, 0.95, 1.0),
                fuzz: 0.,
                refraction_index: 0.,
            },
            uuid: 0,
        },
        // distant moon's moon
        Sphere {
            center: Vec3::new(170., -20., -350.),
            radius: 30.,
            material: Material {
                material_type: MaterialType::Diffuse,
                albedo: Vec3::new(1.0, 1.0, 1.0),
                fuzz: 0.,
                refraction_index: 0.,
            },
            uuid: 0,
        },
    ];

    // @todo: clean this up: make a wrapper type or something
    objects::set_sphere_uuids(&mut sphere_list);

    sphere_list
}
