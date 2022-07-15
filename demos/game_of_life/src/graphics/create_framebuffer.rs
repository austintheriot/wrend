use super::texture_id::TextureId as AppTextureId;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer};
use webgl::renderer::{
    framebuffer_create_context::FramebufferCreateContext, id::Id, id_name::IdName,
};

pub fn make_create_frame_buffer<
    VertexShaderId: Id,
    FragmentShaderId: Id,
    ProgramId: Id,
    UniformId: Id + IdName,
    BufferId: Id + IdName,
    FramebufferId: Id,
    UserCtx: 'static,
>(
    texture_id: AppTextureId,
) -> Rc<
    dyn Fn(
        FramebufferCreateContext<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            AppTextureId,
            FramebufferId,
            UserCtx,
        >,
    ) -> WebGlFramebuffer,
> {
    let callback = move |ctx: FramebufferCreateContext<_, _, _, _, _, _, _, _>| {
        let texture_a = ctx
            .renderer_builder()
            .texture(&texture_id)
            .expect("RendererBuilder should have Texture A built when creating framebuffers");
        let gl = ctx.gl();

        let framebuffer_object = gl
            .create_framebuffer()
            .expect("WebGL2 should be able to create a WebGlFramebuffer object");
        gl.bind_framebuffer(
            WebGl2RenderingContext::FRAMEBUFFER,
            Some(&framebuffer_object),
        );
        gl.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            Some(texture_a.webgl_texture()),
            0,
        );
        framebuffer_object
    };

    Rc::new(callback)
}
