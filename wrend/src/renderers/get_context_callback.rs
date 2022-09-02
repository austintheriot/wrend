use crate::{CallbackWithContext, Either, WebGlContextError};
use js_sys::Function;
use std::fmt::Debug;
use std::{ops::Deref, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub type GetContextCallbackInner = Either<
    CallbackWithContext<
        dyn Fn(HtmlCanvasElement) -> Result<WebGl2RenderingContext, WebGlContextError>,
    >,
    CallbackWithContext<Function>,
>;

/// Wrapper around CallbackWithContext to provide a default implementation
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetContextCallback(GetContextCallbackInner);

impl GetContextCallback {
    pub fn new(callback: impl Into<GetContextCallback>) -> Self {
        callback.into()
    }
}

impl<F: Fn(HtmlCanvasElement) -> Result<WebGl2RenderingContext, WebGlContextError> + 'static>
    From<F> for GetContextCallback
{
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(Rc::new(callback)
            as Rc<
                dyn Fn(HtmlCanvasElement) -> Result<WebGl2RenderingContext, WebGlContextError>,
            >)))
    }
}

impl<F: Fn(HtmlCanvasElement) -> Result<WebGl2RenderingContext, WebGlContextError> + 'static>
    From<Rc<F>> for GetContextCallback
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback
                as Rc<
                    dyn Fn(HtmlCanvasElement) -> Result<WebGl2RenderingContext, WebGlContextError>,
                >,
        )))
    }
}

impl From<Function> for GetContextCallback {
    fn from(callback: Function) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}

impl Default for GetContextCallback {
    fn default() -> Self {
        Self::new(|canvas: HtmlCanvasElement| {
            let gl = canvas
                .get_context("webgl2")
                .map_err(|_| WebGlContextError::RetrievalError)?;

            let gl = gl.ok_or(WebGlContextError::NotFoundError)?;

            let gl: WebGl2RenderingContext = gl
                .dyn_into()
                .map_err(|_| WebGlContextError::TypeConversionError)?;

            Ok(gl)
        })
    }
}

impl Deref for GetContextCallback {
    type Target = GetContextCallbackInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for GetContextCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GetContextCallback").field(&self.0).finish()
    }
}
