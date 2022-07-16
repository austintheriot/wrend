pub type RenderStateCount = u32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderState {
    count: RenderStateCount,
}

impl RenderState {
    pub fn count(&self) -> RenderStateCount {
        self.count
    }

    pub fn inc_count(&mut self) -> &mut Self {
        self.count = self.count.wrapping_add(1);
        self
    }
}
