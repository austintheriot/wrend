use crate::Callback;
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

/// Contains the build information for a WebGL uniform. 
/// 
/// A [`Uniform`] can be associated with any number of programs,
/// and can be updated with [crate::RendererData::update_uniform] or
/// [crate::RendererData::update_uniforms].
#[derive(Clone)]
pub struct Uniform<ProgramId: Id, UniformId: Id> {
    program_ids: Vec<ProgramId>,
    uniform_id: UniformId,
    uniform_locations: HashMap<ProgramId, WebGlUniformLocation>,
    uniform_create_callback: UniformCreateUpdateCallback,
    update_callback: Option<UniformCreateUpdateCallback>,
    should_update_callback: Option<UniformShouldUpdateCallback>,
    use_init_callback_for_update: bool,
}

impl<ProgramId: Id, UniformId: Id> Uniform<ProgramId, UniformId> {
    // @todo move into builder pattern
    pub(crate) fn new(
        program_ids: Vec<ProgramId>,
        uniform_id: UniformId,
        // a single "conceptual" uniform can be shared across multiple programs and updated in tandem
        uniform_locations: HashMap<ProgramId, WebGlUniformLocation>,
        initialize_callback: UniformCreateUpdateCallback,
        update_callback: Option<UniformCreateUpdateCallback>,
        should_update_callback: Option<UniformShouldUpdateCallback>,
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

    /// Gets all program ids associated with this uniform
    pub fn program_ids(&self) -> &Vec<ProgramId> {
        &self.program_ids
    }

    /// Gets this uniform's uniform id
    pub fn uniform_id(&self) -> &UniformId {
        &self.uniform_id
    }

    /// Gets this uniform's location for all associated program ids
    pub fn uniform_locations(&self) -> &HashMap<ProgramId, WebGlUniformLocation> {
        &self.uniform_locations
    }

    /// Gets the callback that is used to initialize this uniform
    pub fn initialize_callback(&self) -> UniformCreateUpdateCallback {
        self.uniform_create_callback.clone()
    }

    /// Gets the callback that is used to determine whether this uniform should update when
    /// [crate::RendererData::update_uniform] or [crate::RendererData::update_uniforms] is called.
    pub fn should_update_callback(&self) -> Option<UniformShouldUpdateCallback> {
        self.should_update_callback.as_ref().map(Clone::clone)
    }

    /// Gets the callback that is used to updated this uniform whenever
    /// [crate::RendererData::update_uniform] or [crate::RendererData::update_uniforms] is called.
    pub fn update_callback(&self) -> Option<UniformCreateUpdateCallback> {
        self.update_callback.as_ref().map(Clone::clone)
    }

    /// If set to `true`, [Uniform] will use the [Uniform::initialize_callback] callback
    /// to update when [crate::RendererData::update_uniform] or [crate::RendererData::update_uniforms] is called
    /// rather than the [Uniform::update_callback]
    pub fn use_init_callback_for_update(&self) -> bool {
        self.use_init_callback_for_update
    }

    /// Updates the value of this uniform in WebGl for every Program where this uniform is used,
    /// using the update callback that was passed in at creation time.
    /// 
    /// @todo: calling this function for anything more than the current program is useless without a UBO
    pub fn update(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        programs: &HashMap<ProgramId, WebGlProgram>,
    ) {
        let uniform_locations = self.uniform_locations();

        for (program_id, uniform_location) in uniform_locations.iter() {
            let program = programs
                .get(program_id)
                .expect("Program id should correspond to a saved WebGlProgram");

            gl.use_program(Some(program));

            let ctx = UniformContext::new(gl.clone(), now, uniform_location.clone());
            let should_update_callback = self.should_update_callback();

            let should_call = if let Some(should_update_callback) = should_update_callback {
                match &*should_update_callback {
                    Callback::Rust(rust_callback) => (rust_callback)(&ctx),
                    Callback::Js(js_callback) => {
                        JsValue::as_bool(&js_callback.call0(&JsValue::NULL).expect(
                            "Should be able to call `should_update_callback` JavaScript callback",
                        ))
                        .unwrap_or(false)
                    }
                }
            } else {
                // by default, assume that all uniforms should be updated, since uniforms should
                // only be updated if no custom optimization callback is provided
                true
            };

            if should_call {
                if self.use_init_callback_for_update {
                    self.uniform_create_callback.call_with_into_js_arg(&ctx);
                } else if let Some(update_callback) = &self.update_callback {
                    update_callback.call_with_into_js_arg(&ctx)
                }
            }

            gl.use_program(None);
        }
    }
}

impl<ProgramId: Id, UniformId: Id> Debug for Uniform<ProgramId, UniformId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Uniform")
            .field("id", &self.uniform_id)
            .field("uniform_locations", &self.uniform_locations)
            .finish()
    }
}
impl<ProgramId: Id, UniformId: Id> Hash for Uniform<ProgramId, UniformId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uniform_id.hash(state);
    }
}

impl<ProgramId: Id, UniformId: Id> PartialEq for Uniform<ProgramId, UniformId> {
    fn eq(&self, other: &Self) -> bool {
        self.uniform_id == other.uniform_id && self.uniform_locations == other.uniform_locations
    }
}

impl<ProgramId: Id, UniformId: Id> Eq for Uniform<ProgramId, UniformId> {}

impl From<UniformJsInner> for JsValue {
    fn from(uniform: UniformJsInner) -> Self {
        let js_uniform: UniformJs = uniform.into();
        js_uniform.into()
    }
}

impl From<UniformJs> for UniformJsInner {
    fn from(js_uniform: UniformJs) -> Self {
        js_uniform.into_inner()
    }
}
