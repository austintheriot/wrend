use super::{id::Id, id_name::IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct IdDefault;

impl Id for IdDefault {}

impl IdName for IdDefault {
    fn name(&self) -> String {
        String::from("id_default")
    }
}
