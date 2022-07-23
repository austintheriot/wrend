use crate::Id;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct TransformFeedbackLink<TransformFeedbackId: Id> {
    transform_feedback_id: TransformFeedbackId,
}

impl<TransformFeedbackId: Id> TransformFeedbackLink<TransformFeedbackId> {
    pub fn new(transform_feedback_id: TransformFeedbackId) -> Self {
        Self {
            transform_feedback_id,
        }
    }
    pub fn transform_feedback_id(&self) -> &TransformFeedbackId {
        &self.transform_feedback_id
    }
}
