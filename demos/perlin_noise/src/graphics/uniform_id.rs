use wrend::{Id, IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    UWhiteNoiseTexture,
    UPerlinNoiseTexture,
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
        }
    }
}
