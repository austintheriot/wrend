use strum::IntoEnumIterator;
use wrend::{ProgramLink, ProgramLinkBuilder};

use super::{FilterType, FragmentShaderId, ProgramId, VertexShaderId, GenerationType};

/// Programmatically creates program links for all filter types
pub fn create_filter_program_links() -> Vec<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>>
{
    let mut program_links = Vec::new();

    for generation_type in FilterType::iter() {
        let mut program_link = ProgramLinkBuilder::new();
        program_link
            .set_vertex_shader_id(VertexShaderId::Quad)
            .set_program_id(generation_type.program_id())
            .set_fragment_shader_id(generation_type.fragment_shader_id());
        let program_link = program_link.build().unwrap_or_else(|_| {
            panic!("Should build program link successfully: {:?}", generation_type)
        });
        program_links.push(program_link);
    }

    program_links
}

/// Programmatically creates program links for all generation types
pub fn create_generate_program_links() -> Vec<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>>
{
    let mut program_links = Vec::new();

    for generation_type in GenerationType::iter() {
        let mut program_link = ProgramLinkBuilder::new();
        program_link
            .set_vertex_shader_id(VertexShaderId::Quad)
            .set_program_id(generation_type.program_id())
            .set_fragment_shader_id(generation_type.fragment_shader_id());
        let program_link = program_link.build().unwrap_or_else(|_| {
            panic!("Should build program link successfully: {:?}", generation_type)
        });
        program_links.push(program_link);
    }

    program_links
}
