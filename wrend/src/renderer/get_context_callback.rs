use crate::{CallbackWithContext, WebGlContextError};
use std::fmt::Debug;
use std::{ops::Deref, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

/// Wrapper around CallbackWithContext to provide a default implementation
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetContextCallback(
    CallbackWithContext<HtmlCanvasElement, Result<WebGl2RenderingContext, WebGlContextError>>,
);

impl GetContextCallback {
    pub fn new(
        callback: impl Into<
            CallbackWithContext<
                HtmlCanvasElement,
                Result<WebGl2RenderingContext, WebGlContextError>,
            >,
        >,
    ) -> Self {
        Self(callback.into())
    }
}

impl<F: Fn(&HtmlCanvasElement) -> Result<WebGl2RenderingContext, WebGlContextError> + 'static>
    From<F> for GetContextCallback
{
    fn from(callback: F) -> Self {
        Self(CallbackWithContext::new(callback))
    }
}

impl<F: Fn(&HtmlCanvasElement) -> Result<WebGl2RenderingContext, WebGlContextError> + 'static>
    From<Rc<F>> for GetContextCallback
{
    fn from(callback: Rc<F>) -> Self {
        Self(CallbackWithContext::new(callback))
    }
}

impl From<CallbackWithContext<HtmlCanvasElement, Result<WebGl2RenderingContext, WebGlContextError>>>
    for GetContextCallback
{
    fn from(
        callback: CallbackWithContext<
            HtmlCanvasElement,
            Result<WebGl2RenderingContext, WebGlContextError>,
        >,
    ) -> Self {
        Self(callback)
    }
}

impl Default for GetContextCallback {
    fn default() -> Self {
        Self(CallbackWithContext::new(|canvas: &HtmlCanvasElement| {
            let gl = canvas
                .get_context("webgl2")
                .map_err(|_| WebGlContextError::RetrievalError)?;

            let gl = gl.ok_or(WebGlContextError::NotFoundError)?;

            let gl: WebGl2RenderingContext = gl
                .dyn_into()
                .map_err(|_| WebGlContextError::TypeConversionError)?;

            Ok(gl)
        }))
    }
}

impl Deref for GetContextCallback {
    type Target =
        CallbackWithContext<HtmlCanvasElement, Result<WebGl2RenderingContext, WebGlContextError>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for GetContextCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GetContextCallback").field(&self.0).finish()
    }
}
