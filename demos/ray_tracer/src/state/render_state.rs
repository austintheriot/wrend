use std::f64::consts::PI;
use web_sys::{HtmlCanvasElement, WebGlTexture, WebGl2RenderingContext};
use wrend::{degrees_to_radians, Vec3};

use crate::{objects::{Sphere, Material, MaterialType, self, HitResult, Ray}, controls::KeydownMap, utils};

pub type RenderStateCount = u32;

pub const MOVEMENT_SPEED: f64 = 0.001;

/// This must match the amount in the shader itself
/// @todo programmatically set the value in the shader before compiling
/// Or use the `MAX_FRAGMENT_UNIFORM_VECTORS` provided by the WebGL context
pub const MAX_NUM_SPHERES: u8 = 15;

/// so high that it's unlikely to be a real id of an object in the shader
pub const NO_SELECTED_OBJECT_ID: i32 = 1000;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RenderState {
    count: u32,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub focal_length: f64,
    pub camera_origin: Vec3,
    pub pitch: f64,
    pub yaw: f64,
    pub camera_front: Vec3,
    pub vup: Vec3,
    /// stored in radians
    pub camera_field_of_view: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub aperture: f64,
    pub lens_radius: f64,
    pub focus_distance: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    pub sphere_list: Vec<Sphere>,

    // RENDER STATE
    /// is the modal up that asks the user to enable first-person viewing mode?
    pub is_paused: bool,
    /// If the render should render incrementally, pubaveraging together previous frames
    pub should_average: bool,
    /// Unless averaging is taking place, pubthis is set to false after revery render
    /// only updated back to true if something changes (i.e. input)
    pub should_render: bool,
    /// Whether the browser should save a screenshot of the canvas
    pub should_save: bool,
    /// Used to alternate which framebuffer to render to
    pub even_odd_count: u32,
    /// Used for averaging previous frames together
    pub render_count: u32,
    /// The weight of the last frame compared to the each frame before.
    pub last_frame_weight: f32,
    /// Limiting the counted renders allows creating a sliding average of frames
    pub max_render_count: u32,
    /// Used for calculating time delta in animation loop
    pub prev_now: f64,
    /// this is necessary after the user resizes their viewport
    pub should_update_to_match_window_size: bool,
    pub last_resize_time: f64,

    // MOVEMENT
    pub keydown_map: KeydownMap,
    pub look_sensitivity: f64,

    // DEBUGGING
    pub enable_debugging: i32,
    pub cursor_point: Vec3,
    pub selected_object: i32,

    // ANALYTICS
    pub prev_fps_update_time: f64,
    pub prev_fps: [f64; 50],
}

impl Default for RenderState {
    fn default() -> Self {
        let (width, height) = utils::clamped_screen_dimensions();
        let aspect_ratio = (width as f64) / (height as f64);
        let aperture = 0.;
        let focus_distance = 0.75;
        let lens_radius = aperture / 2.0;

        let camera_field_of_view = PI / 3.;
        let camera_h = (camera_field_of_view / 2.).tan();
        let camera_origin = Vec3::new(0., 0., 1.);
        let pitch = 0.;
        let yaw = -90.; // look down the z axis by default
        let camera_front = Vec3::new(
            f64::cos(degrees_to_radians(yaw)) * f64::cos(degrees_to_radians(pitch)),
            f64::sin(degrees_to_radians(pitch)),
            f64::sin(degrees_to_radians(yaw)) * f64::cos(degrees_to_radians(pitch)),
        );
        let look_at = &camera_origin + &camera_front;
        let vup = Vec3::new(0., 1., 0.);
        let w = Vec3::normalize(&camera_origin - &look_at);
        let u = Vec3::normalize(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);
        let viewport_height = 2. * camera_h;
        let viewport_width = viewport_height * aspect_ratio;
        let horizontal = focus_distance * viewport_width * &u;
        let vertical = focus_distance * viewport_height * &v;
        let focal_length = 1.;
        let lower_left_corner =
            &camera_origin - &horizontal / 2. - &vertical / 2. - focus_distance * &w;

        let samples_per_pixel = 1;
        let max_depth = 8;
        let should_average = false;
        let should_render = true;
        let should_save = false;
        let even_odd_count = 0;
        let render_count = 0;
        let last_frame_weight = 1.;
        let max_render_count = 100_000;
        let prev_now = 0.;
        let should_update_to_match_window_size = false;
        let last_resize_time = 0.;

        let is_paused = false;

        let look_sensitivity = 0.1;
        let keydown_map = KeydownMap::default();

        let prev_fps_update_time = 0.;
        let prev_fps = [0.; 50];

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

        let enable_debugging = 0;
        let cursor_point = Vec3::new(0., 0., 0.);
        let selected_object = NO_SELECTED_OBJECT_ID;

        objects::set_sphere_uuids(&mut sphere_list);

        RenderState {
            width,
            height,
            aperture,
            u,
            v,
            w,
            focus_distance,
            lens_radius,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            focal_length,
            pitch,
            yaw,
            camera_origin,
            camera_front,
            vup,
            camera_field_of_view,
            viewport_height,
            viewport_width,
            horizontal,
            vertical,
            lower_left_corner,

            is_paused,
            should_average,
            should_render,
            should_save,
            even_odd_count,
            render_count,
            last_frame_weight,
            max_render_count,
            prev_now,
            should_update_to_match_window_size,
            last_resize_time,

            prev_fps_update_time,
            prev_fps,

            keydown_map,
            look_sensitivity,

            enable_debugging,
            cursor_point,
            selected_object,

            sphere_list,
            count: 0,
        }
    }
}

impl RenderState {
    pub fn count(&self) -> RenderStateCount {
        self.count
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn inc_count(&mut self) -> &mut Self {
        self.count = self.count.wrapping_add(1);
        self
    }

    // updates all "downstream" variables once a rendering/camera variable has been changed
    pub fn update_pipeline(&mut self) {
        // for comparing if any changes occured
        let prev_state = self.clone();

        self.aspect_ratio = (self.width as f64) / (self.height as f64);
        let camera_h = (self.camera_field_of_view / 2.).tan();
        self.camera_front = Vec3::new(
            f64::cos(degrees_to_radians(self.yaw)) * f64::cos(degrees_to_radians(self.pitch)),
            f64::sin(degrees_to_radians(self.pitch)),
            f64::sin(degrees_to_radians(self.yaw)) * f64::cos(degrees_to_radians(self.pitch)),
        );
        let look_at = &self.camera_origin + &self.camera_front;
        self.w = Vec3::normalize(&self.camera_origin - &look_at);
        self.u = Vec3::normalize(Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);
        self.viewport_height = 2. * camera_h;
        self.viewport_width = self.viewport_height * self.aspect_ratio;
        self.horizontal = self.focus_distance * self.viewport_width * &self.u;
        self.vertical = self.focus_distance * self.viewport_height * &self.v;
        self.lower_left_corner = &self.camera_origin
            - &self.horizontal / 2.
            - &self.vertical / 2.
            - self.focus_distance * &self.w;

        if self != &prev_state {
            self.render_count = 0;
            self.should_render = true;
        }
    }

    pub fn set_fov(&mut self, new_fov_radians: f64) {
        self.camera_field_of_view = new_fov_radians.clamp(0.0001, PI * 0.75);
        self.update_pipeline();
    }

    pub fn set_camera_angles(&mut self, yaw: f64, pitch: f64) {
        self.yaw = yaw;
        self.pitch = f64::clamp(pitch, -89., 89.);
        self.update_pipeline();
    }
}


pub fn update_render_dimensions_to_match_window(
    state: &mut RenderState,
    gl: &WebGl2RenderingContext,
    textures: &[WebGlTexture; 2],
    canvas: &HtmlCanvasElement,
    now: f64,
) {
    // update state
    state.last_resize_time = now;
    let (width, height) = utils::clamped_screen_dimensions();
    state.width = width;
    state.height = height;
    state.update_pipeline();

    // sync width/height-dependent objects with state
    canvas.set_width(state.width);
    canvas.set_height(state.height);
    gl.viewport(0, 0, state.width as i32, state.height as i32);
    for texture in textures.iter() {
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));
        // load empty texture into gpu -- this will get rendered into later
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            state.width as i32,
            state.height as i32,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        )
        .unwrap();
    }
}

pub fn update_moving_fps_array(now: f64, state: &mut RenderState, dt: f64) {
    // calculate moving fps
    state.prev_now = now;
    let fps = 1000. / dt;
    let last_index = state.prev_fps.len() - 1;
    for (i, el) in state.prev_fps.into_iter().skip(1).enumerate() {
        state.prev_fps[i] = el;
    }
    state.prev_fps[last_index] = fps;
}

pub fn update_position(state: &mut RenderState, dt: f64) {
    if state.keydown_map.all_false() {
        return;
    }

    let camera_front = state.camera_front.clone();
    let vup = state.vup.clone();
    // move slower when more "zoomed in"
    let fov = state.camera_field_of_view;
    if state.keydown_map.w {
        state.camera_origin += &camera_front * MOVEMENT_SPEED * dt * fov;
    }
    if state.keydown_map.a {
        state.camera_origin -= Vec3::cross(&camera_front, &vup) * MOVEMENT_SPEED * dt * fov;
    }
    if state.keydown_map.s {
        state.camera_origin -= &camera_front * MOVEMENT_SPEED * dt * fov;
    }
    if state.keydown_map.d {
        state.camera_origin += Vec3::cross(&camera_front, &vup) * MOVEMENT_SPEED * dt * fov;
    }
    if state.keydown_map.space {
        state.camera_origin += &vup * MOVEMENT_SPEED * dt * fov;
    }
    if state.keydown_map.shift {
        state.camera_origin -= &vup * MOVEMENT_SPEED * dt * fov;
    }

    update_cursor_position_in_world(state);
    state.update_pipeline();
}

pub fn update_render_globals(state: &mut RenderState) {
    if !state.should_average {
        // only continuously render when averaging is being done
        state.should_render = false;
    }
    state.even_odd_count += 1;
    state.render_count = (state.render_count + 1).min(state.max_render_count);
}

/// focus on whatever object is selected by the cursor if there was a collision
pub fn update_cursor_position_in_world(state: &mut RenderState) {
    let ray = Ray {
        origin: state.camera_origin.clone(),
        direction: &state.lower_left_corner + &state.horizontal / 2. + &state.vertical / 2.
            - &state.camera_origin,
    };

    let spheres = &state.sphere_list;

    if let HitResult::Hit { data } = objects::get_center_hit(spheres, ray) {
        let distance = (&data.hit_point - &state.camera_origin).length();
        if state.aperture > 0. {
            // there is no blurring if aperture is zerp
            state.focus_distance = distance;
        }
        state.cursor_point = data.hit_point.clone();
        state.selected_object = data.uuid;
    } else {
        if state.aperture > 0. {
            // there is no blurring if aperture is zerp
            state.focus_distance = 10.;
        }
        state.cursor_point = Vec3::new(0., 0., 0.);
        state.selected_object = NO_SELECTED_OBJECT_ID;
    }
    state.update_pipeline();
}
