use strum::IntoEnumIterator;
use wrend::{ProgramLink, ProgramLinkBuilder};

use super::{FilterType, FragmentShaderId, ProgramId, VertexShaderId};

/// Programmatically creates program links for all filter types
pub fn create_program_links() -> Vec<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>> {
    let mut program_links = Vec::new();

    for filter_type in FilterType::iter() {
        let mut program_link = ProgramLinkBuilder::new();
        program_link
            .set_vertex_shader_id(VertexShaderId::Quad)
            .set_program_id(filter_type.program_id())
            .set_fragment_shader_id(filter_type.fragment_shader_id());
        let program_link = program_link.build().expect(&format!(
            "Should build program link successfully: {:?}",
            filter_type
        ));
        program_links.push(program_link);
    }

    program_links
}
