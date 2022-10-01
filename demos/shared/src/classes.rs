use std::fmt::Display;

pub enum Class {
    SharedButton
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
        Class::SharedButton =>  write!(f, "shared-button"),
    }
    }
}