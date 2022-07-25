use super::{program_id::ProgramId, texture_id::TextureId};
use crate::state::{render_state::MAX_NUM_SPHERES, state_handle::StateHandle};
use std::{rc::Rc, vec};
use wrend::{UniformCallback, UniformContext, UniformLink};

const RAY_TRACER_PROGRAM_ID: ProgramId = ProgramId::RayTracer;

/// Programmatically create uniforms for every possible WebGL sphere
pub fn create_sphere_uniform_links() -> Vec<UniformLink<ProgramId, String, StateHandle>> {
    let mut sphere_uniforms = Vec::new();
    for i in 0..(MAX_NUM_SPHERES as usize) {
        let u_sphere_center_link = UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            format!("u_sphere_list[{}].center", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = app_state_ref.render_state();
                let sphere = &render_state.sphere_list.get(i);
                if let Some(sphere) = sphere {
                    let sphere_center: [f32; 3] = sphere.center.into();
                    gl.uniform3fv_with_f32_array(
                        Some(&uniform_location),
                        &sphere_center.as_slice(),
                    );
                }
            })),
        );
        sphere_uniforms.push(u_sphere_center_link);

        let u_sphere_radius_link = UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            format!("u_sphere_list[{}].radius", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = app_state_ref.render_state();
                let sphere = &render_state.sphere_list.get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1f(Some(&uniform_location), sphere.radius as f32);
                }
            })),
        );
        sphere_uniforms.push(u_sphere_radius_link);

        let u_sphere_material_type_link = UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            format!("u_sphere_list[{}].material.type", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = app_state_ref.render_state();
                let sphere = &render_state.sphere_list.get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1i(
                        Some(&uniform_location),
                        sphere.material.material_type.value(),
                    );
                }
            })),
        );
        sphere_uniforms.push(u_sphere_material_type_link);

        let u_sphere_material_albedo = UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            format!("u_sphere_list[{}].material.albedo", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = app_state_ref.render_state();
                let sphere = &render_state.sphere_list.get(i);
                if let Some(sphere) = sphere {
                    gl.uniform3fv_with_f32_array(
                        Some(&uniform_location),
                        &sphere.material.albedo.to_f32_array().as_slice(),
                    );
                }
            })),
        );
        sphere_uniforms.push(u_sphere_material_albedo);

        let u_sphere_material_fuzz_link = UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            format!("u_sphere_list[{}].material.fuzz", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let sphere = &app_state_ref.render_state().sphere_list.get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1f(Some(&uniform_location), sphere.material.fuzz);
                }
            })),
        );
        sphere_uniforms.push(u_sphere_material_fuzz_link);

        let u_sphere_material_refraction_index_link = UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            format!("u_sphere_list[{}].material.refraction_index", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let sphere = &app_state_ref.render_state().sphere_list.get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1f(Some(&uniform_location), sphere.material.refraction_index);
                }
            })),
        );
        sphere_uniforms.push(u_sphere_material_refraction_index_link);

        let u_sphere_is_active_link = UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            format!("u_sphere_list[{}].is_active", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let sphere = &app_state_ref.render_state().sphere_list.get(i);
                let active_state = sphere.map(|_| 1).unwrap_or(0);
                gl.uniform1i(Some(&uniform_location), active_state);
            })),
        );
        sphere_uniforms.push(u_sphere_is_active_link);

        let u_sphere_uuid_link = UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            format!("u_sphere_list[{}].uuid", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let sphere = &app_state_ref.render_state().sphere_list.get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1i(Some(&uniform_location), sphere.uuid as i32);
                }
            })),
        );
        sphere_uniforms.push(u_sphere_uuid_link);
    }

    sphere_uniforms
}

pub fn create_general_uniform_links() -> Vec<UniformLink<ProgramId, String, StateHandle>>  {
    let u_now_link_init_and_update_callback = Rc::new(|ctx: &UniformContext<StateHandle>| {
        let gl = ctx.gl();
        let uniform_location = ctx.uniform_location();
        gl.uniform1f(Some(uniform_location), (ctx.now() / 2000.) as f32);
    });

    let mut u_now_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_now"),
        UniformCallback::new(u_now_link_init_and_update_callback.clone()),
    );

    u_now_link.set_update_callback(UniformCallback::new(
        u_now_link_init_and_update_callback.clone(),
    ));

    vec![u_now_link]
}

pub fn create_general_ray_tracer_uniform_links() -> Vec<UniformLink<ProgramId, String, StateHandle>> {
    vec![
        UniformLink::new(
            ProgramId::PassThrough,
            String::from("u_averaged_render_texture_a"),
            UniformCallback::new(Rc::new(|ctx| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1ui(
                    Some(uniform_location),
                    TextureId::AveragedRenderA.location(),
                );
            })),
        ),
        UniformLink::new(
            ProgramId::PassThrough,
            String::from("u_averaged_render_texture_b"),
            UniformCallback::new(Rc::new(|ctx| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1ui(
                    Some(uniform_location),
                    TextureId::AveragedRenderB.location(),
                );
            })),
        ),
        UniformLink::new(
            ProgramId::PassThrough,
            String::from("u_prev_render_texture"),
            UniformCallback::new(Rc::new(|ctx| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1ui(Some(uniform_location), TextureId::PrevRender.location());
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_width"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1f(Some(&uniform_location), render_state.width as f32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_height"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1f(Some(&uniform_location), render_state.height as f32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_samples_per_pixel"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                // @todo: revisit this
                //  increase sample rate when paused (such as on first render and when resizing)
                // it's ok to do some heavy lifting here, since it's not being continually rendered at this output
                let samples_per_pixel = if render_state.is_paused {
                    render_state.samples_per_pixel.max(25)
                } else {
                    render_state.samples_per_pixel
                };
                gl.uniform1i(Some(&uniform_location), samples_per_pixel as i32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_aspect_ratio"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1f(Some(&uniform_location), render_state.aspect_ratio as f32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_viewport_height"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1f(Some(&uniform_location), render_state.viewport_height as f32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_viewport_width"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1f(Some(&uniform_location), render_state.viewport_width as f32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_focal_length"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1f(Some(&uniform_location), render_state.focal_length as f32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_camera_origin"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform3fv_with_f32_array(
                    Some(&uniform_location),
                    &render_state.camera_origin.to_f32_array(),
                );
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_horizontal"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform3fv_with_f32_array(
                    Some(&uniform_location),
                    &render_state.horizontal.to_f32_array(),
                );
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_vertical"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform3fv_with_f32_array(
                    Some(&uniform_location),
                    &render_state.vertical.to_f32_array(),
                );
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_lower_left_corner"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform3fv_with_f32_array(
                    Some(&uniform_location),
                    &render_state.lower_left_corner.to_f32_array(),
                );
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_max_depth"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1i(Some(&uniform_location), render_state.max_depth as i32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_render_count"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1i(Some(&uniform_location), render_state.render_count as i32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_should_average"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1i(Some(&uniform_location), render_state.should_average as i32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_last_frame_weight"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1f(
                    Some(&uniform_location),
                    render_state.last_frame_weight as f32,
                );
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_lens_radius"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1f(Some(&uniform_location), render_state.lens_radius as f32);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_u"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform3fv_with_f32_array(
                    Some(&uniform_location),
                    &render_state.u.to_f32_array(),
                );
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_v"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform3fv_with_f32_array(
                    Some(&uniform_location),
                    &render_state.v.to_f32_array(),
                );
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_w"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform3fv_with_f32_array(
                    Some(&uniform_location),
                    &render_state.w.to_f32_array(),
                );
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_selected_object"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1i(Some(&uniform_location), render_state.selected_object);
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_cursor_point"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform3fv_with_f32_array(
                    Some(&uniform_location),
                    &render_state.cursor_point.to_f32_array(),
                );
            })),
        ),
        UniformLink::new(
            RAY_TRACER_PROGRAM_ID,
            String::from("u_enable_debugging"),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<StateHandle>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let app_state_ref = user_ctx.borrow();
                let render_state = &app_state_ref.render_state();
                gl.uniform1i(Some(&uniform_location), render_state.enable_debugging);
            })),
        ),
    ]
}
