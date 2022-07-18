/// Useful for Ids that need a literal String representation (such as uniforms)
pub trait IdName {
    fn name(&self) -> String;
}

impl IdName for String {
    fn name(&self) -> String {
        self.to_owned()
    }
}

impl IdName for &'static str {
    fn name(&self) -> String {
        self.to_string()
    }
}