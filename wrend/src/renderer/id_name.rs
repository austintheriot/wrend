/// Useful for Ids that need a literal String representation (such as uniforms)
pub trait IdName {
    fn name(&self) -> String;
}
