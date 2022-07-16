use super::{id::Id, id_name::IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct DefaultId;

impl Id for DefaultId {}

impl IdName for DefaultId {
    fn name(&self) -> String {
        String::from("default_id")
    }
}
