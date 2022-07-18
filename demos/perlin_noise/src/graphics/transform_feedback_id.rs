use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TransformFeedbackId {
    Default,
}

impl Id for TransformFeedbackId {}

impl Default for TransformFeedbackId {
    fn default() -> Self {
        Self::Default
    }
}
