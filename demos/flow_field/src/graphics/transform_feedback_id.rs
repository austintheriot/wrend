use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TransformFeedbackId {
    Particle,
}

impl Id for TransformFeedbackId {}

impl Default for TransformFeedbackId {
    fn default() -> Self {
        Self::Particle
    }
}
