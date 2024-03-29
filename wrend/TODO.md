# Wrend

The intent for this crate is to serve as a reusable, generic utility crate for doing WebGl rendering.

## Use Cashes

The following use cases should guide the development of this library:

- The ability to save uniform locations and update all uniforms programmatically.
- Perform transform feedback, where all data stays on the GPU
  - Particle systems
- Render to a framebuffer and then retrieve previous render for sampling
  - Conway's Game of Life
  - Any iterative kernel-based rendering
- Ray tracing
- General 2D / 3D rendering

## Todo

- Loose road map:
  - Add tests
  - Improve error messages
  - Make renderer runtime agnostic (i.e. allow native OpenGL as well as WebGL contexts)
  - Allow build to be async ? (this would require `async`s to permeate the library until Rust implements the ability to be generic over `async`/`sync`)
  - Add Vec / Matrix utilities
  - Build out a more structured Scene Graphs, etc. ?
  - Enable more run time options:
    - Enable compiling new shaders / programs
    - Enable updating buffers

- Use ok_or_else on errors to prevent unnecessary work

- Add event system for emitting events related to changes of internal state

- Add note that users should make should they perform a render immediately before calling .save_image() (or prevent buffer clearing) so that the saved image isn't empty

- Make it more explicit when structs get cloned to convert them into a `JsValue`

- Make a procedural macro and/or a custom derive macro for `Id`, and possible `IdName`

- Make some impl that do not need to be called from outside pub(crate)
  - Especially the `new` functions (or the structs themselves?) of Buffer, Uniform, etc. (internal types not built directly)

- Move `js_conversion` utils into a generic trait impl on a new type around Vec, then impl `From` for that type so that the trait can be applied to plain `Vec`s

- Bug: WebGL2 rendering contexts are not getting discarded when handle is dropped: enable this manually through browser extension (see MDN WebGL best practices article)

- Do not use dynamic functions for callbacks--use generics all the way down?
  - (model after Yew's `Callback` type--using generic instead of dynamic dispatch)

- Clean up "into" ergonomics around animation_handle and recording_handle

- Make using uniform links more ergonomic: use builder pattern, etc.

- Make a trait for Texture numbers that is available from the RendererData?

- Cleanup:
  - Consume links when they are used during build time - would require less cloning in general
  - Enable borrowing in context structs - would also require less cloning to occur

- Return error when a duplicate item is added to HashMap?

## Nice to Haves

- Make everything truly as modular as possible
  - Make it renderer_data agnostic?
  - Make recording codec customizable