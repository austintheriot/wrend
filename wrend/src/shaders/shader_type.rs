/// Safe wrapper around WebGL's fragment shader `i32`s
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

impl From<ShaderType> for u32 {
    fn from(shader_type: ShaderType) -> Self {
        match shader_type {
            ShaderType::VertexShader => 35_633,
            ShaderType::FragmentShader => 35_632,
        }
    }
}
