use wrend::renderer::{id::Id, id_name::IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    UTexture,
}

impl Id for UniformId {}

impl Default for UniformId {
    fn default() -> Self {
        Self::UTexture
    }
}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::UTexture => "u_texture".to_string(),
        }
    }
}
