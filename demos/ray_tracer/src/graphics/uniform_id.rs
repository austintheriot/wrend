use wrend::{Id, IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    URenderATexture,
    URenderBTexture,
    UNow,
}

impl Id for UniformId {}

impl Default for UniformId {
    fn default() -> Self {
        Self::URenderBTexture
    }
}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::URenderATexture => "u_render_a_texture".to_string(),
            UniformId::URenderBTexture => "u_render_b_texture".to_string(),
            UniformId::UNow => "u_now".to_string(),
        }
    }
}
