use crate::{CallbackWithContext, Either, TextureCreateContext};
use js_sys::Function;
use std::fmt::Debug;
use std::{ops::Deref, rc::Rc};
use web_sys::WebGlTexture;

type TextureCreateCallbackInner<UserCtx> = Either<
    CallbackWithContext<dyn Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture>,
    CallbackWithContext<Function>,
>;

#[derive(Clone, Hash, Eq, PartialOrd, Ord)]
pub struct TextureCreateCallback<UserCtx>(TextureCreateCallbackInner<UserCtx>);

impl<UserCtx> PartialEq for TextureCreateCallback<UserCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<UserCtx> Deref for TextureCreateCallback<UserCtx> {
    type Target = TextureCreateCallbackInner<UserCtx>;

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
        Self(Either::new_a(CallbackWithContext::from(Rc::new(callback)
            as Rc<
                dyn Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture,
            >)))
    }
}

impl<UserCtx, F: Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture + 'static> From<Rc<F>>
    for TextureCreateCallback<UserCtx>
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&TextureCreateContext<UserCtx>) -> WebGlTexture>,
        )))
    }
}

impl<UserCtx> From<Function> for TextureCreateCallback<UserCtx> {
    fn from(callback: Function) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}
