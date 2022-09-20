use wrend::Id;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum VAOId {
    #[default]
    Quad,
}

impl Id for VAOId {}