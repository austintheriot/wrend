use std::fmt::Debug;
use std::hash::Hash;

/// This contains an id for a pair of shaders: one vertex shader and one fragment
/// shader. These can be combined to link together a program.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ProgramLink<I>
where
    I: Hash + Eq + Clone + Debug,
{
    vertex_shader_id: I,
    fragment_shader_id: I,
}

impl<I> ProgramLink<I>
where
    I: Hash + Eq + Clone + Debug,
{
    pub fn new(vertex_shader_id: I, fragment_shader_id: I) -> Self {
        Self {
            vertex_shader_id,
            fragment_shader_id,
        }
    }

    pub fn vertex_shader_id(&self) -> &I {
        &self.vertex_shader_id
    }

    pub fn fragment_shader_id(&self) -> &I {
        &self.fragment_shader_id
    }
}
