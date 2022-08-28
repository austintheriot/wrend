use crate::Id;
use crate::UniformContext;
use crate::UniformCreateUpdateCallback;
use crate::UniformJs;
use crate::UniformJsInner;
use crate::UniformShouldUpdateCallback;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

#[derive(Clone)]
pub struct Uniform<ProgramId: Id, UniformId: Id, UserCtx: Clone> {
    program_ids: Vec<ProgramId>,
    uniform_id: UniformId,
    uniform_locations: HashMap<ProgramId, WebGlUniformLocation>,
    uniform_create_callback: UniformCreateUpdateCallback<UserCtx>,
    update_callback: Option<UniformCreateUpdateCallback<UserCtx>>,
    should_update_callback: Option<UniformShouldUpdateCallback<UserCtx>>,
    use_init_callback_for_update: bool,
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> Uniform<ProgramId, UniformId, UserCtx> {
    // @todo move into builder pattern
    pub(crate) fn new(
        program_ids: Vec<ProgramId>,
        uniform_id: UniformId,
        // a single "conceptual" uniform can be shared across multiple programs and updated in tandem
        uniform_locations: HashMap<ProgramId, WebGlUniformLocation>,
        initialize_callback: UniformCreateUpdateCallback<UserCtx>,
        update_callback: Option<UniformCreateUpdateCallback<UserCtx>>,
        should_update_callback: Option<UniformShouldUpdateCallback<UserCtx>>,
        use_init_callback_for_update: bool,
    ) -> Self {
        Self {
            program_ids,
            uniform_id,
            uniform_locations,
            uniform_create_callback: initialize_callback,
            update_callback,
            should_update_callback,
            use_init_callback_for_update,
        }
    }

    pub fn program_ids(&self) -> &Vec<ProgramId> {
        &self.program_ids
    }

    pub fn uniform_id(&self) -> &UniformId {
        &self.uniform_id
    }

    pub fn uniform_locations(&self) -> &HashMap<ProgramId, WebGlUniformLocation> {
        &self.uniform_locations
    }

    pub fn initialize_callback(&self) -> UniformCreateUpdateCallback<UserCtx> {
        self.uniform_create_callback.clone()
    }

    pub fn should_update_callback(&self) -> Option<UniformShouldUpdateCallback<UserCtx>> {
        self.should_update_callback.as_ref().map(Clone::clone)
    }

    pub fn update_callback(&self) -> Option<UniformCreateUpdateCallback<UserCtx>> {
        self.update_callback.as_ref().map(Clone::clone)
    }

    /// updates the uniform for every Program where this uniform is used,
    /// using the update callback that was passed in at creation time
    pub fn update(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        user_ctx: Option<UserCtx>,
        programs: &HashMap<ProgramId, WebGlProgram>,
    ) {
        let uniform_locations = self.uniform_locations();

        for (program_id, uniform_location) in uniform_locations.iter() {
            let user_ctx = user_ctx.clone();
            let program = programs
                .get(program_id)
                .expect("Program id should correspond to a saved WebGlProgram");

            gl.use_program(Some(program));

            let ctx = UniformContext::new(gl.clone(), now, uniform_location.clone(), user_ctx);
            let should_update_callback = self.should_update_callback().unwrap_or_default();

            let should_call = match &*should_update_callback {
                crate::Either::A(rust_callback) => (rust_callback)(&ctx),
                crate::Either::B(js_callback) => {
                    JsValue::as_bool(&js_callback.call0(&JsValue::NULL).expect(
                        "Should be able to call `should_update_callback` JavaScript callback",
                    ))
                    .unwrap_or(false)
                }
            };

            if should_call {
                if self.use_init_callback_for_update {
                    self.uniform_create_callback.call(&ctx);
                } else if let Some(update_callback) = &self.update_callback {
                    update_callback.call(&ctx)
                }
            }

            gl.use_program(None);
        }
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> Debug
    for Uniform<ProgramId, UniformId, UserCtx>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Uniform")
            .field("id", &self.uniform_id)
            .field("uniform_locations", &self.uniform_locations)
            .finish()
    }
}
impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> Hash for Uniform<ProgramId, UniformId, UserCtx> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uniform_id.hash(state);
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> PartialEq
    for Uniform<ProgramId, UniformId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.uniform_id == other.uniform_id && self.uniform_locations == other.uniform_locations
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> Eq for Uniform<ProgramId, UniformId, UserCtx> {}

impl From<UniformJsInner> for JsValue {
    fn from(uniform: UniformJsInner) -> Self {
        let js_uniform: UniformJs = uniform.into();
        js_uniform.into()
    }
}

impl From<UniformJs> for UniformJsInner {
    fn from(js_uniform: UniformJs) -> Self {
        js_uniform.inner()
    }
}
