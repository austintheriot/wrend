use crate::{CallbackWithContext, TextureCreateContext};
use web_sys::WebGlTexture;

pub type TextureCreateCallback<UserCtx> =
    CallbackWithContext<TextureCreateContext<UserCtx>, WebGlTexture>;
