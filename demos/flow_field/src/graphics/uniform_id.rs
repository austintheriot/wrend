use wrend::{Id, IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    UNoiseTexture,
}

impl Id for UniformId {}

impl Default for UniformId {
    fn default() -> Self {
        Self::UNoiseTexture
    }
}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::UNoiseTexture => "u_noise_texture".to_string(),
        }
    }
}
