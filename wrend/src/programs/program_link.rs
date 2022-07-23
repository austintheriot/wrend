use crate::Id;
use std::fmt::Debug;
use std::hash::Hash;
use thiserror::Error;

/// This contains an id for a pair of shaders: one vertex shader and one fragment
/// shader. These can be combined to link together a program.
#[derive(Clone, Debug)]
pub struct ProgramLink<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id> {
    program_id: ProgramId,
    vertex_shader_id: VertexShaderId,
    fragment_shader_id: FragmentShaderId,
    transform_feedback_varyings: Vec<String>,
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id>
    ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>
{
    pub fn new(
        program_id: ProgramId,
        vertex_shader_id: VertexShaderId,
        fragment_shader_id: FragmentShaderId,
    ) -> Self {
        Self {
            program_id,
            vertex_shader_id,
            fragment_shader_id,
            transform_feedback_varyings: Default::default(),
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

    pub fn transform_feedback_varyings(&self) -> &[String] {
        &self.transform_feedback_varyings
    }

    pub fn builder() -> ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId> {
        ProgramLinkBuilder::default()
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id> Hash
    for ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.vertex_shader_id.hash(state);
        self.fragment_shader_id.hash(state);
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id> PartialEq
    for ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.vertex_shader_id == other.vertex_shader_id
            && self.fragment_shader_id == other.fragment_shader_id
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id> Eq
    for ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>
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
pub struct ProgramLinkBuilder<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id> {
    program_id: Option<ProgramId>,
    vertex_shader_id: Option<VertexShaderId>,
    fragment_shader_id: Option<FragmentShaderId>,
    transform_feedback_varyings: Vec<String>,
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id>
    ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId>
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

    pub fn set_transform_feedback_varyings(
        mut self,
        transform_feedback_varyings: impl Into<Vec<String>>,
    ) -> Self {
        self.transform_feedback_varyings = transform_feedback_varyings.into();
        self
    }

    pub fn build(
        self,
    ) -> Result<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>, ProgramLinkBuildError>
    {
        Ok(ProgramLink {
            program_id: self.program_id.ok_or(ProgramLinkBuildError::NoProgramId)?,
            vertex_shader_id: self
                .vertex_shader_id
                .ok_or(ProgramLinkBuildError::NoVertexShaderId)?,
            fragment_shader_id: self
                .fragment_shader_id
                .ok_or(ProgramLinkBuildError::NoFragmentShaderId)?,
            transform_feedback_varyings: self.transform_feedback_varyings,
        })
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id> Default
    for ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId>
{
    fn default() -> Self {
        Self {
            program_id: Default::default(),
            vertex_shader_id: Default::default(),
            fragment_shader_id: Default::default(),
            transform_feedback_varyings: Default::default(),
        }
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id> Hash
    for ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.vertex_shader_id.hash(state);
        self.fragment_shader_id.hash(state);
    }
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id> PartialEq
    for ProgramLinkBuilder<ProgramId, VertexShaderId, FragmentShaderId>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.vertex_shader_id == other.vertex_shader_id
            && self.fragment_shader_id == other.fragment_shader_id
    }
}
