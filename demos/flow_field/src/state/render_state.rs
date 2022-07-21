use crate::graphics::buffer_id::BufferId;

use super::read_write_buffers::ReadWriteBuffers;

pub type RenderStateCount = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderState {
    num_particles: u32,
    count: u32,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            num_particles: 250_000,
            count: 0,
        }
    }
}

impl RenderState {
    pub fn num_particles(&self) -> u32 {
        self.num_particles
    }

    pub fn num_particle_vertices(&self) -> u32 {
        self.num_particles * 3
    }

    pub fn next_read_write_buffers(&mut self) -> ReadWriteBuffers {
        let read_write_buffers = if self.count % 2 == 0 {
            ReadWriteBuffers::new(
                BufferId::ParticleBufferA,
                BufferId::ParticleBufferB,
            )
        } else {
            ReadWriteBuffers::new(
                BufferId::ParticleBufferB,
                BufferId::ParticleBufferA,
            )
        };

        self.count = self.count.wrapping_add(1);

        read_write_buffers
    }
}
