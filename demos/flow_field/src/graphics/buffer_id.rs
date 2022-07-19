use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BufferId {
    QuadVertexBuffer,
    ParticleBufferA,
    ParticleBufferB,
}

impl Id for BufferId {}

impl Default for BufferId {
    fn default() -> Self {
        Self::QuadVertexBuffer
    }
}
