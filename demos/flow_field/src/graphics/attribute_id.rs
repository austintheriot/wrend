use wrend::{Id, IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AttributeId {
    AQuadVertex,
    AParticlePositionA,
    AParticlePositionB,
}

impl Id for AttributeId {}

impl IdName for AttributeId {
    fn name(&self) -> String {
        match self {
            AttributeId::AQuadVertex => String::from("a_quad_vertex"),
            AttributeId::AParticlePositionA => String::from("a_particle_position"),
            AttributeId::AParticlePositionB => String::from("a_particle_position"),
        }
    }
}

impl Default for AttributeId {
    fn default() -> Self {
        AttributeId::AQuadVertex
    }
}