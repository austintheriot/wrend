use crate::graphics::buffer_id::BufferId;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct ReadWriteBuffers {
    read_buffer: BufferId,
    write_buffer: BufferId,
}

impl ReadWriteBuffers {
    pub fn read_buffer(&self) -> BufferId {
        self.read_buffer
    }

    pub fn write_buffer(&self) -> BufferId {
        self.write_buffer
    }

    pub fn new(read_buffer: BufferId, write_buffer: BufferId) -> Self {
        Self { read_buffer, write_buffer }
    }
}
