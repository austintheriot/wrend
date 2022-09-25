use wrend::{Id, IdName};

// Because I'm using dynamically set uniforms,
// it's necessary to use raw strings for UniformIds, but
// Specifying some statically known uniform ids is still helpful
// for mitigating type errors
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    #[default]
    USrcTexture,
    USrcVideo,
    UNow,
}

impl Id for UniformId {}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::USrcTexture => "u_src_texture".to_string(),
            UniformId::USrcVideo => "u_src_video".to_string(),
            UniformId::UNow => "u_now".to_string(),
        }
    }
}
