use crate::{CallbackWithContext, TextureCreateContext};
use std::fmt::Debug;
use std::{ops::Deref, rc::Rc};
use web_sys::WebGlTexture;

#[derive(Clone, Hash, Eq, PartialOrd, Ord)]
pub struct TextureCreateCallback<UserCtx>(
    CallbackWithContext<dyn Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture>,
);

impl<UserCtx> PartialEq for TextureCreateCallback<UserCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<UserCtx> Deref for TextureCreateCallback<UserCtx> {
    type Target = CallbackWithContext<dyn Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UserCtx> Debug for TextureCreateCallback<UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TextureCreateCallback")
            .field(&self.0)
            .finish()
    }
}

impl<UserCtx, F: Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture + 'static> From<F>
    for TextureCreateCallback<UserCtx>
{
    fn from(callback: F) -> Self {
        Self(CallbackWithContext::from(Rc::new(callback)
            as Rc<
                dyn Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture,
            >))
    }
}

impl<UserCtx, F: Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture + 'static> From<Rc<F>>
    for TextureCreateCallback<UserCtx>
{
    fn from(callback: Rc<F>) -> Self {
        Self(CallbackWithContext::from(
            callback as Rc<dyn Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture>,
        ))
    }
}
