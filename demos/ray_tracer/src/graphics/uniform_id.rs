use wrend::{Id, IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    UPrevRenderTexture,
    UAveragedRenderTextureA,
    UAveragedRenderTextureB,
    UNow,
}

impl Id for UniformId {}

impl Default for UniformId {
    fn default() -> Self {
        Self::UPrevRenderTexture
    }
}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::UPrevRenderTexture => "u_prev_render_texture".to_string(),
            UniformId::UAveragedRenderTextureA => "u_averaged_render_texture".to_string(),
            UniformId::UAveragedRenderTextureB => "u_averaged_render_texture".to_string(),
            UniformId::UNow => "u_now".to_string(),
        }
    }
}
