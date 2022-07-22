use crate::graphics::{buffer_id::BufferId, vao_id::VAOId};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct ReadWriteIds {
    read_ids: (BufferId, VAOId),
    write_ids: (BufferId, VAOId),
}

impl ReadWriteIds {
    pub fn read_ids(&self) -> (BufferId, VAOId) {
        self.read_ids
    }

    pub fn write_ids(&self) -> (BufferId, VAOId) {
        self.write_ids
    }

    pub fn new(read_ids: (BufferId, VAOId), write_ids: (BufferId, VAOId)) -> Self {
        Self {
            read_ids,
            write_ids,
        }
    }
}
