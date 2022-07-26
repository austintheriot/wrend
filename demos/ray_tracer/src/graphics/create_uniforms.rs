use super::{program_id::ProgramId, texture_id::TextureId};
use crate::state::{app_context::AppContext, render_state::MAX_NUM_SPHERES};
use std::{rc::Rc, vec};
use wrend::{UniformCallback, UniformContext, UniformLink};

/// Programmatically create uniforms for every possible WebGL sphere
pub fn create_sphere_uniform_links() -> Vec<UniformLink<ProgramId, String, AppContext>> {
    let mut sphere_uniforms = Vec::with_capacity(MAX_NUM_SPHERES as usize);
    for i in 0..(MAX_NUM_SPHERES as usize) {
        let mut uniform_link = UniformLink::new(
            ProgramId::RayTracer,
            format!("u_sphere_list[{}].center", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let render_state = user_ctx.render_state.borrow();
                let sphere = &render_state.sphere_list().get(i);
                if let Some(sphere) = sphere {
                    let sphere_center: [f32; 3] = sphere.center.into();
                    gl.uniform3fv_with_f32_array(Some(uniform_location), sphere_center.as_slice());
                }
            })),
        );
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = UniformLink::new(
            ProgramId::RayTracer,
            format!("u_sphere_list[{}].radius", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let render_state = user_ctx.render_state.borrow();
                let sphere = &render_state.sphere_list().get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1f(Some(uniform_location), sphere.radius as f32);
                }
            })),
        );
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = UniformLink::new(
            ProgramId::RayTracer,
            format!("u_sphere_list[{}].material.type", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let render_state = user_ctx.render_state.borrow();
                let sphere = &render_state.sphere_list().get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1i(
                        Some(uniform_location),
                        sphere.material.material_type.value(),
                    );
                }
            })),
        );
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = UniformLink::new(
            ProgramId::RayTracer,
            format!("u_sphere_list[{}].material.albedo", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let render_state = user_ctx.render_state.borrow();
                let sphere = &render_state.sphere_list().get(i);
                if let Some(sphere) = sphere {
                    gl.uniform3fv_with_f32_array(
                        Some(uniform_location),
                        sphere.material.albedo.to_f32_array().as_slice(),
                    );
                }
            })),
        );
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = UniformLink::new(
            ProgramId::RayTracer,
            format!("u_sphere_list[{}].material.fuzz", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let render_state = user_ctx.render_state.borrow();
                let sphere = &render_state.sphere_list().get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1f(Some(uniform_location), sphere.material.fuzz);
                }
            })),
        );
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = UniformLink::new(
            ProgramId::RayTracer,
            format!("u_sphere_list[{}].material.refraction_index", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let render_state = user_ctx.render_state.borrow();
                let sphere = &render_state.sphere_list().get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1f(Some(uniform_location), sphere.material.refraction_index);
                }
            })),
        );
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = UniformLink::new(
            ProgramId::RayTracer,
            format!("u_sphere_list[{}].is_active", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let render_state = user_ctx.render_state.borrow();
                let sphere = &render_state.sphere_list().get(i);
                let active_state = sphere.map(|_| 1).unwrap_or(0);
                gl.uniform1i(Some(uniform_location), active_state);
            })),
        );
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = UniformLink::new(
            ProgramId::RayTracer,
            format!("u_sphere_list[{}].uuid", i),
            UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let user_ctx = ctx.user_ctx().unwrap();
                let render_state = user_ctx.render_state.borrow();
                let sphere = &render_state.sphere_list().get(i);
                if let Some(sphere) = sphere {
                    gl.uniform1i(Some(uniform_location), sphere.uuid as i32);
                }
            })),
        );
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);
    }

    sphere_uniforms
}

pub fn create_general_uniform_links() -> Vec<UniformLink<ProgramId, String, AppContext>> {
    vec![
        UniformLink::new(
            ProgramId::AverageRenders,
            String::from("u_averaged_render_texture_a"),
            UniformCallback::new(Rc::new(|ctx| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1i(
                    Some(uniform_location),
                    TextureId::AveragedRenderA.location() as i32,
                );
            })),
        ),
        UniformLink::new(
            ProgramId::AverageRenders,
            String::from("u_averaged_render_texture_b"),
            UniformCallback::new(Rc::new(|ctx| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1i(
                    Some(uniform_location),
                    TextureId::AveragedRenderB.location() as i32,
                );
            })),
        ),
        UniformLink::new(
            ProgramId::AverageRenders,
            String::from("u_prev_render_texture"),
            UniformCallback::new(Rc::new(|ctx| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1i(
                    Some(uniform_location),
                    TextureId::PrevRender.location() as i32,
                );
            })),
        ),
    ]
}

pub fn create_general_ray_tracer_uniform_links() -> Vec<UniformLink<ProgramId, String, AppContext>>
{
    let mut general_ray_tracer_uniform_links = Vec::new();

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_now"),
        UniformCallback::new(Rc::new(|ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            gl.uniform1f(Some(uniform_location), (ctx.now() / 2000.) as f32);
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_width"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            let pipeline = render_state.camera();
            gl.uniform1f(Some(uniform_location), pipeline.width() as f32);
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_height"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            let pipeline = render_state.camera();
            gl.uniform1f(Some(uniform_location), pipeline.height() as f32);
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_samples_per_pixel"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            gl.uniform1i(
                Some(uniform_location),
                render_state.samples_per_pixel() as i32,
            );
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    // let mut uniform_link = UniformLink::new(
    //     ProgramId::RayTracer,
    //     String::from("u_aspect_ratio"),
    //     UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
    //         let gl = ctx.gl();
    //         let uniform_location = ctx.uniform_location();
    //         let user_ctx = ctx.user_ctx().unwrap();
    //         let render_state = user_ctx.render_state.borrow();
    //         gl.uniform1f(Some(&uniform_location), render_state.aspect_ratio as f32);
    //     })),
    // ),
    // let mut uniform_link = UniformLink::new(
    //     ProgramId::RayTracer,
    //     String::from("u_viewport_height"),
    //     UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
    //         let gl = ctx.gl();
    //         let uniform_location = ctx.uniform_location();
    //         let user_ctx = ctx.user_ctx().unwrap();
    //         let render_state = user_ctx.render_state.borrow();
    //         gl.uniform1f(Some(&uniform_location), render_state.viewport_height as f32);
    //     })),
    // ),
    // let mut uniform_link = UniformLink::new(
    //     ProgramId::RayTracer,
    //     String::from("u_viewport_width"),
    //     UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
    //         let gl = ctx.gl();
    //         let uniform_location = ctx.uniform_location();
    //         let user_ctx = ctx.user_ctx().unwrap();
    //         let render_state = user_ctx.render_state.borrow();
    //         gl.uniform1f(Some(&uniform_location), render_state.viewport_width as f32);
    //     })),
    // ),
    // let mut uniform_link = UniformLink::new(
    //     ProgramId::RayTracer,
    //     String::from("u_focal_length"),
    //     UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
    //         let gl = ctx.gl();
    //         let uniform_location = ctx.uniform_location();
    //         let user_ctx = ctx.user_ctx().unwrap();
    //         let render_state = user_ctx.render_state.borrow();
    //         gl.uniform1f(Some(&uniform_location), render_state.focal_length as f32);
    //     })),
    // ),
    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_camera_origin"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            let pipeline = render_state.camera();
            gl.uniform3fv_with_f32_array(
                Some(uniform_location),
                &pipeline.camera_origin().to_f32_array(),
            );
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_horizontal"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            let pipeline = render_state.camera();
            gl.uniform3fv_with_f32_array(
                Some(uniform_location),
                &pipeline.horizontal().to_f32_array(),
            );
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_vertical"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            let pipeline = render_state.camera();
            gl.uniform3fv_with_f32_array(
                Some(uniform_location),
                &pipeline.vertical().to_f32_array(),
            );
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_lower_left_corner"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            let pipeline = render_state.camera();
            gl.uniform3fv_with_f32_array(
                Some(uniform_location),
                &pipeline.lower_left_corner().to_f32_array(),
            );
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_max_depth"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            gl.uniform1i(Some(uniform_location), render_state.max_depth() as i32);
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    // let mut uniform_link = UniformLink::new(
    //     ProgramId::RayTracer,
    //     String::from("u_render_count"),
    //     UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
    //         let gl = ctx.gl();
    //         let uniform_location = ctx.uniform_location();
    //         let user_ctx = ctx.user_ctx().unwrap();
    //         let render_state = user_ctx.render_state.borrow();
    //         gl.uniform1i(Some(&uniform_location), render_state.render_count as i32);
    //     })),
    // ),
    // let mut uniform_link = UniformLink::new(
    //     ProgramId::RayTracer,
    //     String::from("u_last_frame_weight"),
    //     UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
    //         let gl = ctx.gl();
    //         let uniform_location = ctx.uniform_location();
    //         let user_ctx = ctx.user_ctx().unwrap();
    //         let render_state = user_ctx.render_state.borrow();
    //         gl.uniform1f(
    //             Some(&uniform_location),
    //             render_state.last_frame_weight as f32,
    //         );
    //     })),
    // ),
    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_lens_radius"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            let pipeline = render_state.camera();
            gl.uniform1f(Some(uniform_location), pipeline.lens_radius() as f32);
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_u"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            let pipeline = render_state.camera();
            gl.uniform3fv_with_f32_array(Some(uniform_location), &pipeline.u().to_f32_array());
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_v"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            let pipeline = render_state.camera();
            gl.uniform3fv_with_f32_array(Some(uniform_location), &pipeline.v().to_f32_array());
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    // let mut uniform_link = UniformLink::new(
    //     ProgramId::RayTracer,
    //     String::from("u_w"),
    //     UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
    //         let gl = ctx.gl();
    //         let uniform_location = ctx.uniform_location();
    //         let user_ctx = ctx.user_ctx().unwrap();
    //         let render_state = user_ctx.render_state.borrow();
    //         gl.uniform3fv_with_f32_array(
    //             Some(&uniform_location),
    //             &render_state.w.to_f32_array(),
    //         );
    //     })),
    // ),
    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_selected_object"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            gl.uniform1i(Some(uniform_location), render_state.selected_object());
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_cursor_point"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            gl.uniform3fv_with_f32_array(
                Some(uniform_location),
                &render_state.cursor_point().to_f32_array(),
            );
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = UniformLink::new(
        ProgramId::RayTracer,
        String::from("u_enable_debugging"),
        UniformCallback::new(Rc::new(move |ctx: &UniformContext<AppContext>| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            let user_ctx = ctx.user_ctx().unwrap();
            let render_state = user_ctx.render_state.borrow();
            gl.uniform1i(Some(uniform_location), render_state.debugging_enabled());
        })),
    );
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    general_ray_tracer_uniform_links
}
