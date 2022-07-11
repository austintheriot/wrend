use std::hash::Hash;
use std::fmt::Debug;

/// Collection of traits that make something usable as an id for programs, shaders, uniforms, etc.
pub trait Id: Hash + Eq + Clone + Debug + Default {}