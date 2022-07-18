use super::program_create_callback::ProgramCreateCallback;
use crate::Id;
use std::fmt::Debug;
use std::hash::Hash;
use thiserror::Error;

/// This contains an id for a pair of shaders: one vertex shader and one fragment
/// shader. These can be combined to link together a program.
#[derive(Clone, Debug)]
pub struct ProgramLink<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx> {
    program_id: ProgramId,
    vertex_shader_id: VertexShaderId,
    fragment_shader_id: FragmentShaderId,
    program_create_callback: ProgramCreateCallback<UserCtx>,
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx>
    ProgramLink<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>
{
    pub fn new(
        program_id: ProgramId,
        vertex_shader_id: VertexShaderId,
        fragment_shader_id: FragmentShaderId,
        program_create_callback: ProgramCreateCallback<UserCtx>,
    ) -> Self {
        Self {
            program_id,
            vertex_shader_id,
            fragment_shader_id,
            program_create_callback,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn vertex_shader_id(&self) -> &VertexShaderId {
        &self.vertex_shader_id
    }

    pub fn fragment_shader_id(&self) -> &FragmentShaderId {
        &self.fragment_shader_id
    }

    pub fn builder() -> ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId, UserCtx> {
        ProgramLinkBuilder::default()
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx> Hash
    for ProgramLink<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.vertex_shader_id.hash(state);
        self.fragment_shader_id.hash(state);
        self.program_create_callback.hash(state);
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx> PartialEq
    for ProgramLink<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.vertex_shader_id == other.vertex_shader_id
            && self.fragment_shader_id == other.fragment_shader_id
            && *self.program_create_callback == *other.program_create_callback
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx> Eq
    for ProgramLink<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>
{
}

#[derive(Error, Debug)]
pub enum ProgramLinkBuildError {
    #[error("No VertexShaderId was supplied")]
    NoVertexShaderId,
    #[error("No FragmentShaderId was supplied")]
    NoFragmentShaderId,
    #[error("No ProgramId was supplied")]
    NoProgramId,
}

#[derive(Clone)]
pub struct ProgramLinkBuilder<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx> {
    program_id: Option<ProgramId>,
    vertex_shader_id: Option<VertexShaderId>,
    fragment_shader_id: Option<FragmentShaderId>,
    program_create_callback: ProgramCreateCallback<UserCtx>,
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx>
    ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_program_id(mut self, program_id: ProgramId) -> Self {
        self.program_id = Some(program_id);
        self
    }

    pub fn set_vertex_shader_id(mut self, vertex_shader_id: VertexShaderId) -> Self {
        self.vertex_shader_id = Some(vertex_shader_id);
        self
    }

    pub fn set_fragment_shader_id(mut self, fragment_shader_id: FragmentShaderId) -> Self {
        self.fragment_shader_id = Some(fragment_shader_id);
        self
    }

    pub fn set_program_create_callback(
        mut self,
        program_create_callback: ProgramCreateCallback<UserCtx>,
    ) -> Self {
        self.program_create_callback = program_create_callback;
        self
    }

    pub fn build(
        self,
    ) -> Result<
        ProgramLink<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>,
        ProgramLinkBuildError,
    > {
        Ok(ProgramLink {
            program_id: self.program_id.ok_or(ProgramLinkBuildError::NoProgramId)?,
            vertex_shader_id: self
                .vertex_shader_id
                .ok_or(ProgramLinkBuildError::NoVertexShaderId)?,
            fragment_shader_id: self
                .fragment_shader_id
                .ok_or(ProgramLinkBuildError::NoFragmentShaderId)?,
            program_create_callback: self.program_create_callback,
        })
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx> Default
    for ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>
{
    fn default() -> Self {
        Self {
            program_id: Default::default(),
            vertex_shader_id: Default::default(),
            fragment_shader_id: Default::default(),
            program_create_callback: Default::default(),
        }
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx> Hash
    for ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.vertex_shader_id.hash(state);
        self.fragment_shader_id.hash(state);
        self.program_create_callback.hash(state);
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id, UserCtx> PartialEq
    for ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.vertex_shader_id == other.vertex_shader_id
            && self.fragment_shader_id == other.fragment_shader_id
            && *self.program_create_callback == *other.program_create_callback
    }
}
