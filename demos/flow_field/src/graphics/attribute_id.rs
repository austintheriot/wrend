use wrend::renderer::{id_name::IdName, id::Id};

#[derive(Clone, Default, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AttributeId;

impl Id for AttributeId {}

impl IdName for AttributeId {
    fn name(&self) -> String {
        String::from("a_position")
    }
}