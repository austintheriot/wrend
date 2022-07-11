use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGlUniformLocation, WebGl2RenderingContext};

use super::uniform_context::UniformContext;

pub struct Uniform<Id, UserCtx>
where
    Id: Hash + Eq + Clone + Debug + Default,
{
    location: WebGlUniformLocation,
    id: Id,
    uniform_location: WebGlUniformLocation,
    update_callback: Box<dyn Fn(&UniformContext<UserCtx>)>
}
