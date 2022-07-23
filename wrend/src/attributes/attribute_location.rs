/// Wrapper around the raw number returned from WebGL to represent an attribute location
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeLocation(u32);

impl AttributeLocation {
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl From<i32> for AttributeLocation {
    fn from(attribute_location: i32) -> Self {
        AttributeLocation(attribute_location as u32)
    }
}

impl From<u32> for AttributeLocation {
    fn from(attribute_location: u32) -> Self {
        AttributeLocation(attribute_location)
    }
}

impl From<&u32> for AttributeLocation {
    fn from(attribute_location: &u32) -> Self {
        AttributeLocation(*attribute_location)
    }
}

impl From<AttributeLocation> for i32 {
    fn from(attribute_location: AttributeLocation) -> Self {
        attribute_location.0 as i32
    }
}

impl From<AttributeLocation> for u32 {
    fn from(attribute_location: AttributeLocation) -> Self {
        attribute_location.0
    }
}

impl From<&AttributeLocation> for u32 {
    fn from(attribute_location: &AttributeLocation) -> Self {
        attribute_location.0
    }
}
