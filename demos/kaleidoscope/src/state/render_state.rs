use crate::graphics::{FilterType, GenerationType};


use super::RenderCycle;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct RenderState {
    filter_type: FilterType,
    generation_type: GenerationType,
    render_cycle: RenderCycle,
}

impl RenderState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filter_type(&self) -> &FilterType {
        &self.filter_type
    }

    pub fn set_filter_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
    }

    pub fn generation_type(&self) -> &GenerationType {
        &self.generation_type
    }

    pub fn set_generation_type(&mut self, generation_type: GenerationType) {
        self.generation_type = generation_type;
    }

    pub fn render_cycle(&self) -> &RenderCycle {
        &self.render_cycle
    }

    pub fn set_render_cycle(&mut self, render_cycle: RenderCycle) {
        self.render_cycle = render_cycle;
    }
}
