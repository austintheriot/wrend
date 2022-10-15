use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum SharedClass {
    Button,
}

impl From<SharedClass> for &str {
    fn from(shared_class: SharedClass) -> Self {
        match shared_class {
            SharedClass::Button => "shared-button",
        }
    }
}

impl Display for SharedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class_as_string: &'static str = self.into();
        write!(f, "{}", class_as_string)
    }
}

impl From<&SharedClass> for &str {
    fn from(shared_class: &SharedClass) -> Self {
        match shared_class {
            SharedClass::Button => (*shared_class).into(),
        }
    }
}
