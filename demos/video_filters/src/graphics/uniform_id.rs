use wrend::{Id, IdName};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    #[default]
    USrcVideoTexture,
    UPrevRenderTextureA,
    UPrevRenderTextureB,
    UNow,
}

impl Id for UniformId {}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::USrcVideoTexture => "u_src_video_texture".to_string(),
            UniformId::UPrevRenderTextureA => "u_prev_render_texture_a".to_string(),
            UniformId::UPrevRenderTextureB => "u_prev_render_texture_b".to_string(),
            UniformId::UNow => "u_now".to_string(),
        }
    }
}
