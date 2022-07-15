use webgl::renderer::id::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ProgramId {
    GameOfLife,
    PassThrough
}

impl Id for ProgramId {}

impl Default for ProgramId {
    fn default() -> Self {
        Self::GameOfLife
    }
} 


