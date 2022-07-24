use crate::Vec3;
use std::ops::Deref;

#[derive(Clone, PartialEq, Copy, Debug, Default, PartialOrd)]
pub struct Color(Vec3);

impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
