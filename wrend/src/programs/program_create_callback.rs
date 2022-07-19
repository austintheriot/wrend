use super::program_create_context::ProgramCreateContext;
use crate::CallbackWithContext;
use std::hash::Hash;
use std::{fmt::Debug, ops::Deref, rc::Rc};
use thiserror::Error;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

#[derive(Error, Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CreateProgramError {
    #[error("Could not create program because value returned by `gl.link_program` was `None`")]
    NoProgramLinkProgramError,
    #[error("Could not create program. Reason: {0}")]
    KnownErrorLinkProgramError(String),
    #[error("Could not create program because varyings should not be converted to an array")]
    CouldNotConvertVaryingsToArray,
    #[error("Could not create program because an unknown error occurred")]
    UnknownErrorLinkProgramError,
}

/// Wrapper around `CallbackWithContext` to implement a custom Default implementation
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ProgramCreateCallback<UserCtx>(
    CallbackWithContext<ProgramCreateContext<UserCtx>, Result<WebGlProgram, CreateProgramError>>,
);

impl<UserCtx> Clone for ProgramCreateCallback<UserCtx> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<UserCtx> Deref for ProgramCreateCallback<UserCtx> {
    type Target = CallbackWithContext<
        ProgramCreateContext<UserCtx>,
        Result<WebGlProgram, CreateProgramError>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UserCtx> Default for ProgramCreateCallback<UserCtx> {
    fn default() -> Self {
        Self(CallbackWithContext::new(Rc::new(|ctx| {
            let gl = ctx.gl();
            let vertex_shader = ctx.vertex_shader();
            let fragment_shader = ctx.fragment_shader();
            let transform_feedback_varyings = ctx.transform_feedback_varyings();

            let webgl_program = gl
                .create_program()
                .ok_or(CreateProgramError::NoProgramLinkProgramError)?;

            gl.attach_shader(&webgl_program, vertex_shader);
            gl.attach_shader(&webgl_program, fragment_shader);

            if !transform_feedback_varyings.is_empty() {
                let varyings_js_value = JsValue::from_serde(transform_feedback_varyings)
                    .map_err(|_| CreateProgramError::CouldNotConvertVaryingsToArray)?;
                gl.transform_feedback_varyings(
                    &webgl_program,
                    &varyings_js_value,
                    WebGl2RenderingContext::INTERLEAVED_ATTRIBS,
                )
            }

            gl.link_program(&webgl_program);

            if gl
                .get_program_parameter(&webgl_program, WebGl2RenderingContext::LINK_STATUS)
                .as_bool()
                .unwrap_or(false)
            {
                Ok(webgl_program)
            } else {
                Err(match gl.get_program_info_log(&webgl_program) {
                    Some(known_error) => {
                        CreateProgramError::KnownErrorLinkProgramError(known_error)
                    }
                    None => CreateProgramError::UnknownErrorLinkProgramError,
                })
            }
        })))
    }
}
