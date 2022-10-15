use crate::graphics::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, vao_id::VAOId, vertex_shader_id::VertexShaderId,
};

use super::{render_state::RenderState, ui_state::UiState};
use std::{cell::RefCell, rc::Rc};
use wrend::Renderer;
use yew::{use_mut_ref, use_reducer_eq, UseReducerHandle};

pub type UiStateHandle = UseReducerHandle<UiState>;
pub type RenderStateHandle = Rc<RefCell<RenderState>>;
pub type AppRenderer = Renderer<
    VertexShaderId,
    FragmentShaderId,
    ProgramId,
    String,
    BufferId,
    AttributeId,
    TextureId,
    FramebufferId,
    TransformFeedbackId,
    VAOId,
    AppContext,
>;
pub type RendererHandle = Rc<RefCell<Option<AppRenderer>>>;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub ui_state: UiStateHandle,
    pub render_state: RenderStateHandle,
    pub renderer: RendererHandle,
}

impl Default for AppContext {
    fn default() -> Self {
        AppContext {
            ui_state: use_reducer_eq(UiState::default),
            render_state: use_mut_ref(RenderState::default),
            renderer: use_mut_ref(Option::default),
        }
    }
}

impl PartialEq for AppContext {
    fn eq(&self, other: &Self) -> bool {
        self.ui_state == other.ui_state
        // ignore render_state and renderer for partial_eq comparisons
    }
}

pub struct AppContextError;

impl AppContextError {
    pub const NOT_FOUND: &'static str = "AppContext was not found";
}
