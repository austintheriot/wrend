use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VAOId {
    PerlinNoise,
    PassThrough,
    UpdateParticlesA,
    UpdateParticlesB,
    DrawParticles,
}

impl Id for VAOId {}

impl Default for VAOId {
    fn default() -> Self {
        Self::PassThrough
    }
}
