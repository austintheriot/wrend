use super::program_create_context::ProgramCreateContext;
use std::hash::Hash;
use std::{any::Any, fmt::Debug, ops::Deref, rc::Rc};
use thiserror::Error;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

#[derive(Error, Debug)]
pub enum CreateProgramError {
    #[error("Could not create program because value returned by `gl.link_program` was `None`")]
    NoProgramLinkProgramError,
    #[error("Could not create program. Reason: {0}")]
    KnownErrorLinkProgramError(String),
    #[error("Could not create program because varyings should not be converted to an array")]
    CouldNotConvertVaryingsToArray,
    #[error("Could not create program because an unknown error occurred")]
    UnknownErrorLinkProgramError,
    #[error("Could not create program: {0:?}")]
    Other(Box<dyn Any>),
}

pub type ProgramCreateCallbackType<UserCtx> =
    dyn Fn(&ProgramCreateContext<UserCtx>) -> Result<WebGlProgram, CreateProgramError>;

pub struct ProgramCreateCallback<UserCtx> {
    program_create_callback: Rc<ProgramCreateCallbackType<UserCtx>>,
    uuid: Uuid,
}

impl<UserCtx> Default for ProgramCreateCallback<UserCtx> {
    fn default() -> Self {
        Self {
            program_create_callback: Rc::new(|ctx| {
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
            }),
            uuid: Uuid::new_v4(),
        }
    }
}

impl<UserCtx> Deref for ProgramCreateCallback<UserCtx> {
    type Target = ProgramCreateCallbackType<UserCtx>;

    fn deref(&self) -> &Self::Target {
        &*self.program_create_callback
    }
}

impl<UserCtx> Clone for ProgramCreateCallback<UserCtx> {
    fn clone(&self) -> Self {
        Self {
            program_create_callback: Rc::clone(&self.program_create_callback),
            uuid: self.uuid.clone(),
        }
    }
}

impl<UserCtx> PartialEq for ProgramCreateCallback<UserCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<UserCtx> Eq for ProgramCreateCallback<UserCtx> {}

impl<UserCtx> Hash for ProgramCreateCallback<UserCtx> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<UserCtx> Debug for ProgramCreateCallback<UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgramCreateCallback")
            .field("program_create_callback", &"[not shown]")
            .field("uuid", &self.uuid)
            .finish()
    }
}

impl<UserCtx> From<Rc<ProgramCreateCallbackType<UserCtx>>> for ProgramCreateCallback<UserCtx> {
    fn from(callback: Rc<ProgramCreateCallbackType<UserCtx>>) -> Self {
        ProgramCreateCallback {
            program_create_callback: callback,
            uuid: Uuid::new_v4(),
        }
    }
}

impl<UserCtx: 'static>
    From<fn(&ProgramCreateContext<UserCtx>) -> Result<WebGlProgram, CreateProgramError>>
    for ProgramCreateCallback<UserCtx>
{
    fn from(
        callback: fn(&ProgramCreateContext<UserCtx>) -> Result<WebGlProgram, CreateProgramError>,
    ) -> Self {
        ProgramCreateCallback {
            program_create_callback: Rc::new(callback),
            uuid: Uuid::new_v4(),
        }
    }
}
