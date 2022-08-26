use crate::{JsAttributeLink, JsProgramLink, JsRenderer, JsTexture, RendererBuilder};
use js_sys::{Function, Object};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;

/// Wrapper around `Renderer` to make it callable from JavaScript.
///
/// Types are adjusted to only use JavaScript-compatible types and no generics.
type JsRendererBuilderInner = RendererBuilder<
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    Object,
>;

#[wasm_bindgen(js_name = RendererBuilder)]
pub struct JsRendererBuilder(JsRendererBuilderInner);

#[wasm_bindgen(js_class = RendererBuilder)]
impl JsRendererBuilder {
    pub fn texture(&self, texture_id: String) -> Option<JsTexture> {
        self.deref().texture(&texture_id).map(Into::into)
    }

    pub fn set_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
        self.deref_mut().set_canvas(canvas);
        self
    }

    pub fn add_fragment_shader_src(mut self, id: String, fragment_shader_src: String) -> Self {
        self.deref_mut()
            .add_fragment_shader_src(id, fragment_shader_src);
        self
    }

    pub fn add_vertex_shader_src(mut self, id: String, vertex_shader_src: String) -> Self {
        self.deref_mut()
            .add_vertex_shader_src(id, vertex_shader_src);
        self
    }

    pub fn add_program_link(mut self, program_link: JsProgramLink) -> Self {
        self.deref_mut().add_program_link(program_link);
        self
    }

    // pub fn add_program_links(
    //     &mut self,
    //     program_links: impl Into<Bridge<ProgramLink<String, String, String>>>,
    // ) -> Self {
    //     let program_link_bridge: Bridge<ProgramLink<String, String, String>> =
    //         program_links.into();
    //     let program_links: Vec<_> = program_link_bridge.into();

    //     for program_link in program_links {
    //         self.add_program_link(program_link);
    //     }

    //     self
    // }

    /// Save a callback that will be called each time it is time to render a new frame
    pub fn set_render_callback(mut self, render_callback: Function) -> Self {
        self.deref_mut().set_render_callback(render_callback);

        self
    }

    /// Save as arbitrary user context that can be accessed from within the render callback
    ///
    /// This can include stateful data and anything else that might be necessary to access
    /// while performing a render.
    pub fn set_user_ctx(mut self, ctx: Object) -> Self {
        self.deref_mut().set_user_ctx(ctx);
        self
    }

    // pub fn add_uniform_link(
    //     &mut self,
    //     uniform_link: JsUniformLink,
    // ) -> Self {
    //     self.deref_mut().add_uniform_link(uniform_link);

    //     self
    // }

    // pub fn add_uniform_links(
    //     &mut self,
    //     uniform_links: impl Into<Bridge<UniformLink<String, UniformId, UserCtx>>>,
    // ) -> Self {
    //     let uniform_link_bridge: Bridge<_> = uniform_links.into();
    //     let uniform_links: Vec<_> = uniform_link_bridge.into();

    //     for uniform_link in uniform_links {
    //         self.add_uniform_link(uniform_link);
    //     }

    //     self
    // }

    /// Saves a link that will be used to build a WebGL buffer at build time.
    // pub fn add_buffer_link(
    //     &mut self,
    //     buffer_link: impl Into<BufferLink<BufferId, UserCtx>>,
    // ) -> Self {
    //     self.buffer_links.insert(buffer_link.into());

    //     self
    // }

    // pub fn add_buffer_links(
    //     &mut self,
    //     buffer_links: impl Into<Bridge<BufferLink<BufferId, UserCtx>>>,
    // ) -> Self {
    //     let buffer_link_bridge: Bridge<_> = buffer_links.into();
    //     let buffer_links: Vec<_> = buffer_link_bridge.into();

    //     for buffer_link in buffer_links {
    //         self.add_buffer_link(buffer_link);
    //     }

    //     self
    // }

    pub fn add_attribute_link(mut self, attribute_link: JsAttributeLink) -> Self {
        self.deref_mut().add_attribute_link(attribute_link);
        self
    }

    // pub fn add_attribute_links(&mut self, attribute_links: Array) -> Self {
    //     let attribute_links: Vec<JsAttributeLink> = js_sys::try_iter(attribute_links.as_ref())
    //     .unwrap()
    //     .expect("add_attribute_links should be passed an array of AttributeLinks")
    //     .into_iter()
    //     .map(|el| {
    //         let js_value = el.unwrap();
    //         let attribute_link: JsAttributeLink = js_value.unchecked_into();
    //         attribute_link
    //     })
    //     .collect();

    //     for attribute_link in attribute_links {
    //         self.add_attribute_link(attribute_link);
    //     }

    //     self
    // }

    /// Saves a link that will be used to build a buffer/attribute pair at build time.
    // pub fn add_texture_link(
    //     &mut self,
    //     texture_link: impl Into<TextureLink<String, UserCtx>>,
    // ) -> Self {
    //     self.texture_links.insert(texture_link.into());

    //     self
    // }

    // pub fn add_texture_links(
    //     &mut self,
    //     texture_links: impl Into<Bridge<TextureLink<String, UserCtx>>>,
    // ) -> Self {
    //     let texture_link_bridge: Bridge<_> = texture_links.into();
    //     let texture_links: Vec<_> = texture_link_bridge.into();

    //     for texture_link in texture_links {
    //         self.add_texture_link(texture_link);
    //     }

    //     self
    // }

    /// Saves a link that will be used to build a framebuffer at build time
    // pub fn add_framebuffer_link(
    //     &mut self,
    //     framebuffer_link: impl Into<FramebufferLink<FramebufferId, UserCtx, String>>,
    // ) -> Self {
    //     self.framebuffer_links.insert(framebuffer_link.into());

    //     self
    // }

    // pub fn add_framebuffer_links(
    //     &mut self,
    //     framebuffer_links: impl Into<Bridge<FramebufferLink<FramebufferId, UserCtx, String>>>,
    // ) -> Self {
    //     let framebuffer_link_bridge: Bridge<_> = framebuffer_links.into();
    //     let framebuffer_links: Vec<_> = framebuffer_link_bridge.into();

    //     for framebuffer_link in framebuffer_links {
    //         self.add_framebuffer_link(framebuffer_link);
    //     }

    //     self
    // }

    /// Saves a link that will be used to build a transformFeedback at build time
    // pub fn add_transform_feedback_link(
    //     &mut self,
    //     transform_feedback_link: impl Into<TransformFeedbackLink<TransformFeedbackId>>,
    // ) -> Self {
    //     self.transform_feedback_links
    //         .insert(transform_feedback_link.into());

    //     self
    // }

    // pub fn add_transform_feedback_links(
    //     &mut self,
    //     transform_feedback_links: impl Into<Bridge<TransformFeedbackLink<TransformFeedbackId>>>,
    // ) -> Self {
    //     let transform_feedback_link_bridge: Bridge<_> = transform_feedback_links.into();
    //     let transform_feedback_links: Vec<_> = transform_feedback_link_bridge.into();

    //     for transform_feedback_link in transform_feedback_links {
    //         self.add_transform_feedback_link(transform_feedback_link);
    //     }

    //     self
    // }

    /// Saves a link that will be used to build a VAO at build time
    // pub fn add_vao_link(
    //     &mut self,
    //     vertex_array_object_id: impl Into<VertexArrayObjectId>,
    // ) -> Self {
    //     self.vertex_array_object_links
    //         .insert(vertex_array_object_id.into());

    //     self
    // }

    // pub fn add_vao_links(
    //     &mut self,
    //     vao_links: impl Into<Bridge<VertexArrayObjectId>>,
    // ) -> Self {
    //     let vao_link_bridge: Bridge<_> = vao_links.into();
    //     let vao_links: Vec<_> = vao_link_bridge.into();
    //     let vao_links: Vec<VertexArrayObjectId> = vao_links.into_iter().collect();

    //     for vao_link in vao_links {
    //         self.add_vao_link(vao_link);
    //     }

    //     self
    // }

    // pub fn set_get_context_callback(
    //     &mut self,
    //     get_context_callback: impl Into<GetContextCallback>,
    // ) -> Self {
    //     self.get_context_callback = get_context_callback.into();
    //     self
    // }

    /// Compiles all vertex shaders and fragment shaders.
    /// Links together any programs that have been specified.
    /// Outputs the final Renderer.
    pub fn build(self) -> Result<JsRenderer, String> {
        self.0
            .build()
            .map(Into::into)
            .map_err(|err| err.to_string())
    }

    pub fn default() -> Self {
        Self(RendererBuilder::default())
    }
}

impl Deref for JsRendererBuilder {
    type Target = JsRendererBuilderInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsRendererBuilder {
    fn deref_mut(&mut self) -> &mut JsRendererBuilderInner {
        &mut self.0
    }
}
