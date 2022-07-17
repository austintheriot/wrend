use wrend::{Id, IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AttributeId {
    APosition,
}

impl Id for AttributeId {}

impl Default for AttributeId {
    fn default() -> Self {
        Self::APosition
    }
}

impl IdName for AttributeId {
    fn name(&self) -> String {
        match self {
            AttributeId::APosition => "a_position".to_string(),
        }
    }
}
