use super::{program_id::ProgramId, texture_id::TextureId};
use crate::state::{app_context::AppContext, render_state::MAX_NUM_SPHERES};
use std::vec;
use wrend::{UniformContext, UniformLink};

/// Programmatically create uniforms for every possible WebGL sphere
pub fn create_sphere_uniform_links(app_context: AppContext) -> Vec<UniformLink<ProgramId, String>> {
    let mut sphere_uniforms = Vec::with_capacity(MAX_NUM_SPHERES as usize);
    for i in 0..(MAX_NUM_SPHERES as usize) {
        let mut uniform_link = {
            let app_context = app_context.clone();
            UniformLink::new(
                ProgramId::RayTracer,
                format!("u_sphere_list[{}].center", i),
                move |ctx: &UniformContext| {
                    let gl = ctx.gl();
                    let uniform_location = ctx.uniform_location();
                    let render_state = app_context.render_state.borrow();
                    let sphere = &render_state.sphere_list().get(i);
                    if let Some(sphere) = sphere {
                        let sphere_center: [f32; 3] = sphere.center.into();
                        gl.uniform3fv_with_f32_array(
                            Some(uniform_location),
                            sphere_center.as_slice(),
                        );
                    }
                },
            )
        };
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = {
            let app_context = app_context.clone();
            UniformLink::new(
                ProgramId::RayTracer,
                format!("u_sphere_list[{}].radius", i),
                move |ctx: &UniformContext| {
                    let gl = ctx.gl();
                    let uniform_location = ctx.uniform_location();
                    let render_state = app_context.render_state.borrow();
                    let sphere = &render_state.sphere_list().get(i);
                    if let Some(sphere) = sphere {
                        gl.uniform1f(Some(uniform_location), sphere.radius as f32);
                    }
                },
            )
        };
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = {
            let app_context = app_context.clone();
            UniformLink::new(
                ProgramId::RayTracer,
                format!("u_sphere_list[{}].material.type", i),
                move |ctx: &UniformContext| {
                    let gl = ctx.gl();
                    let uniform_location = ctx.uniform_location();
                    let render_state = app_context.render_state.borrow();
                    let sphere = &render_state.sphere_list().get(i);
                    if let Some(sphere) = sphere {
                        gl.uniform1i(
                            Some(uniform_location),
                            sphere.material.material_type.value(),
                        );
                    }
                },
            )
        };
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = {
            let app_context = app_context.clone();
            UniformLink::new(
                ProgramId::RayTracer,
                format!("u_sphere_list[{}].material.albedo", i),
                move |ctx: &UniformContext| {
                    let gl = ctx.gl();
                    let uniform_location = ctx.uniform_location();
                    let render_state = app_context.render_state.borrow();
                    let sphere = &render_state.sphere_list().get(i);
                    if let Some(sphere) = sphere {
                        gl.uniform3fv_with_f32_array(
                            Some(uniform_location),
                            sphere.material.albedo.to_f32_array().as_slice(),
                        );
                    }
                },
            )
        };
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = {
            let app_context = app_context.clone();
            UniformLink::new(
                ProgramId::RayTracer,
                format!("u_sphere_list[{}].material.fuzz", i),
                move |ctx: &UniformContext| {
                    let gl = ctx.gl();
                    let uniform_location = ctx.uniform_location();
                    let render_state = app_context.render_state.borrow();
                    let sphere = &render_state.sphere_list().get(i);
                    if let Some(sphere) = sphere {
                        gl.uniform1f(Some(uniform_location), sphere.material.fuzz);
                    }
                },
            )
        };
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = {
            let app_context = app_context.clone();
            UniformLink::new(
                ProgramId::RayTracer,
                format!("u_sphere_list[{}].material.refraction_index", i),
                move |ctx: &UniformContext| {
                    let gl = ctx.gl();
                    let uniform_location = ctx.uniform_location();
                    let render_state = app_context.render_state.borrow();
                    let sphere = &render_state.sphere_list().get(i);
                    if let Some(sphere) = sphere {
                        gl.uniform1f(Some(uniform_location), sphere.material.refraction_index);
                    }
                },
            )
        };
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = {
            let app_context = app_context.clone();
            UniformLink::new(
                ProgramId::RayTracer,
                format!("u_sphere_list[{}].is_active", i),
                move |ctx: &UniformContext| {
                    let gl = ctx.gl();
                    let uniform_location = ctx.uniform_location();
                    let render_state = app_context.render_state.borrow();
                    let sphere = &render_state.sphere_list().get(i);
                    let active_state = sphere.map(|_| 1).unwrap_or(0);
                    gl.uniform1i(Some(uniform_location), active_state);
                },
            )
        };
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);

        let mut uniform_link = {
            let app_context = app_context.clone();
            UniformLink::new(
                ProgramId::RayTracer,
                format!("u_sphere_list[{}].uuid", i),
                move |ctx: &UniformContext| {
                    let gl = ctx.gl();
                    let uniform_location = ctx.uniform_location();
                    let render_state = app_context.render_state.borrow();
                    let sphere = &render_state.sphere_list().get(i);
                    if let Some(sphere) = sphere {
                        gl.uniform1i(Some(uniform_location), sphere.uuid as i32);
                    }
                },
            )
        };
        uniform_link.set_use_init_callback_for_update(true);
        sphere_uniforms.push(uniform_link);
    }

    sphere_uniforms
}

pub fn create_shared_uniform_links(app_context: AppContext) -> Vec<UniformLink<ProgramId, String>> {
    let mut shared_uniform_links = vec![
        UniformLink::new(
            [ProgramId::AverageRenders, ProgramId::PassThrough],
            String::from("u_averaged_render_texture"),
            |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1i(
                    Some(uniform_location),
                    // the location is the same for both `A` and `B`
                    TextureId::AveragedRenderA.location() as i32,
                );
            },
        ),
        UniformLink::new(
            [ProgramId::AverageRenders, ProgramId::PassThrough],
            String::from("u_prev_render_texture"),
            |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1i(
                    Some(uniform_location),
                    TextureId::PrevRender.location() as i32,
                );
            },
        ),
    ];

    let mut uniform_link = {
        let app_context = app_context;
        UniformLink::new(
            [ProgramId::AverageRenders, ProgramId::PassThrough],
            String::from("u_render_count"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                gl.uniform1i(Some(uniform_location), render_state.render_count() as i32);
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    shared_uniform_links.push(uniform_link);

    shared_uniform_links
}

pub fn create_general_ray_tracer_uniform_links(
    app_context: AppContext,
) -> Vec<UniformLink<ProgramId, String>> {
    let mut general_ray_tracer_uniform_links = Vec::new();

    let mut uniform_link = {
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_now"),
            |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1f(Some(uniform_location), (ctx.now() / 2000.) as f32);
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_width"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                let pipeline = render_state.camera();
                gl.uniform1f(Some(uniform_location), pipeline.width() as f32);
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_height"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                let pipeline = render_state.camera();
                gl.uniform1f(Some(uniform_location), pipeline.height() as f32);
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_samples_per_pixel"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                gl.uniform1i(
                    Some(uniform_location),
                    render_state.samples_per_pixel() as i32,
                );
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_camera_origin"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                let pipeline = render_state.camera();
                gl.uniform3fv_with_f32_array(
                    Some(uniform_location),
                    &pipeline.camera_origin().to_f32_array(),
                );
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_horizontal"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                let pipeline = render_state.camera();
                gl.uniform3fv_with_f32_array(
                    Some(uniform_location),
                    &pipeline.horizontal().to_f32_array(),
                );
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_vertical"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                let pipeline = render_state.camera();
                gl.uniform3fv_with_f32_array(
                    Some(uniform_location),
                    &pipeline.vertical().to_f32_array(),
                );
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_lower_left_corner"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                let pipeline = render_state.camera();
                gl.uniform3fv_with_f32_array(
                    Some(uniform_location),
                    &pipeline.lower_left_corner().to_f32_array(),
                );
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_max_depth"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                gl.uniform1i(Some(uniform_location), render_state.max_depth() as i32);
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);
    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_lens_radius"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                let pipeline = render_state.camera();
                gl.uniform1f(Some(uniform_location), pipeline.lens_radius() as f32);
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_u"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                let pipeline = render_state.camera();
                gl.uniform3fv_with_f32_array(Some(uniform_location), &pipeline.u().to_f32_array());
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_v"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                let pipeline = render_state.camera();
                gl.uniform3fv_with_f32_array(Some(uniform_location), &pipeline.v().to_f32_array());
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_selected_object"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                gl.uniform1i(Some(uniform_location), render_state.selected_object());
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context.clone();
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_cursor_point"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                gl.uniform3fv_with_f32_array(
                    Some(uniform_location),
                    &render_state.cursor_point().to_f32_array(),
                );
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    let mut uniform_link = {
        let app_context = app_context;
        UniformLink::new(
            ProgramId::RayTracer,
            String::from("u_enable_debugging"),
            move |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                let render_state = app_context.render_state.borrow();
                gl.uniform1i(Some(uniform_location), render_state.enable_debugging());
            },
        )
    };
    uniform_link.set_use_init_callback_for_update(true);
    general_ray_tracer_uniform_links.push(uniform_link);

    general_ray_tracer_uniform_links
}
