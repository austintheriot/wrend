use crate::{CallbackWithContext, Either, TextureCreateContext, TextureCreateCallbackJs};
use js_sys::Function;
use std::fmt::Debug;
use std::{ops::Deref, rc::Rc};
use web_sys::WebGlTexture;

type TextureCreateCallbackInner = Either<
    CallbackWithContext<dyn Fn(&TextureCreateContext) -> WebGlTexture>,
    CallbackWithContext<TextureCreateCallbackJs>,
>;

#[derive(Clone, Hash, Eq, PartialOrd, Ord)]
pub struct TextureCreateCallback(TextureCreateCallbackInner);

impl PartialEq for TextureCreateCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for TextureCreateCallback {
    type Target = TextureCreateCallbackInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for TextureCreateCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TextureCreateCallback")
            .field(&self.0)
            .finish()
    }
}

impl<F: Fn(&TextureCreateContext) -> WebGlTexture + 'static> From<F> for TextureCreateCallback {
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&TextureCreateContext) -> WebGlTexture>
        )))
    }
}

impl<F: Fn(&TextureCreateContext) -> WebGlTexture + 'static> From<Rc<F>> for TextureCreateCallback {
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&TextureCreateContext) -> WebGlTexture>,
        )))
    }
}

impl From<TextureCreateCallbackJs> for TextureCreateCallback {
    fn from(callback: TextureCreateCallbackJs) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}
