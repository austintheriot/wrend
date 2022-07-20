use wrend::{Id, IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    UWhiteNoiseTexture,
    UPerlinNoiseTexture,
    UPrevFrameTextureA,
    UPrevFrameTextureB,
    UNow,
}

impl Id for UniformId {}

impl Default for UniformId {
    fn default() -> Self {
        Self::UPerlinNoiseTexture
    }
}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::UWhiteNoiseTexture => "u_white_noise_texture".to_string(),
            UniformId::UPerlinNoiseTexture => "u_perlin_noise_texture".to_string(),
            UniformId::UNow => "u_now".to_string(),
            UniformId::UPrevFrameTextureA => "u_prev_frame_texture_a".to_string(),
            UniformId::UPrevFrameTextureB => "u_prev_frame_texture_b".to_string(),
        }
    }
}
