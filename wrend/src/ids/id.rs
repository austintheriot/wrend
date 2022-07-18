use std::fmt::Debug;
use std::hash::Hash;

/// Collection of traits that make something useful as an id for programs, shaders, uniforms, etc.
pub trait Id: 'static + Hash + PartialEq + Eq + Clone + Debug + Default {}

impl Id for String {}

impl Id for &'static str {}

impl Id for u8 {}

impl Id for u16 {}

impl Id for u32 {}

impl Id for u64 {}

impl Id for u128 {}

impl Id for i8 {}

impl Id for i16 {}

impl Id for i32 {}

impl Id for i64 {}

impl Id for i128 {}
