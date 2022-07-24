#[derive(Clone, PartialEq, Debug, Copy, Eq, PartialOrd, Ord)]
pub enum MaterialType {
    Diffuse,
    Metal,
    Glass,
}

impl MaterialType {
    pub fn value(&self) -> i32 {
        match self {
            MaterialType::Diffuse => 0,
            MaterialType::Metal => 1,
            MaterialType::Glass => 2,
        }
    }
}

impl Default for MaterialType {
    fn default() -> Self {
        MaterialType::Diffuse
    }
}