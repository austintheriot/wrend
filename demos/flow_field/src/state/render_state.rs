use crate::graphics::buffer_id::BufferId;

use super::read_write_buffers::ReadWriteBuffers;

pub type RenderStateCount = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderState {
    num_particles: u32,
    count: u32,
    should_save_image: bool,
    is_first_render: bool,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            num_particles: 100_000,
            count: 0,
            should_save_image: false,
            is_first_render: true,
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

    pub fn should_save_image(&self) -> bool {
        self.should_save_image
    }

    pub fn set_should_save_image(&mut self, should_save_image: bool) -> &mut Self {
        self.should_save_image = should_save_image;
        self
    }

    pub fn is_first_render(&self) -> bool {
        self.is_first_render
    }

    pub fn set_is_first_render(&mut self, is_first_render: bool) -> &mut Self {
        self.is_first_render = is_first_render;
        self
    }
}
