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

## Todo

- Make it more explicit when structs get cloned to convert them into a `JsValue`

- Implementing JavaScript API
  - Make sure all public JavaScript API struct docs point / link to their respective Rust counterparts for documentation
  - Derive all traits for Js structs

- Make a procedural macro and/or a custom derive macro for `Id`, and possible `IdName`

- Make some impl that do not need to be called from outside pub(crate)
  - Especially the `new` functions (or the structs themselves?) of Buffer, Uniform, etc. (internal types not built directly)

- Move `js_conversion` utils into a generic trait impl on a new type around Vec, then impl `From` for that type so that the trait can be applied to plain `Vec`s

- Bug: WebGL2 rendering contexts are not getting discarded when handle is dropped: enable this manually through browser extension (see MDN WebGL best practices article)

- Only set up RecordingData when requested

- Do not use dynamic functions for callbacks

- Enable recording video from canvas
  - Enable starting/stopping recording and downloading as separate steps

- Add event system for emitting events related to changes of internal state

- Clean up "into" ergonomics around animation_handle and recording_handle

- Fix callback system (model after Yew's `Callback` type--using generic instead of dynamic dispatch)

- Make using uniform links more ergonomic: use builder pattern, etc.

- Make a trait for Texture numbers that is available from the RendererData?

- Cleanup:
  - Consume links when they are used during build time - would require less cloning in general
  - Enable borrowing in context structs - would also require less cloning to occur

- Road map:
  - Add Vec / Matrix utilities
  - Build out a more structured Scene Graphs, etc. ?
  - Enable compiling new shaders / programs at run time ?

- Enable transform feedback

- Make passing in buffer update / should_update callbacks optional

- Return error when a duplicate item is added to HashMap?

## Nice to Haves

- Make everything truly as modular as possible
  - Make it renderer_data agnostic?
  - Make recording codec customizable

- Make cross-compatible /callable with vanilla JavaScript
  - This would require making sure all returned handles can cross the WasmAbi
  - It would also probably require a refactor of callbacks, so that plain JavaScript closures could be accepted as well
    - This could be achieved with a enum for callbacks (one for plain Rust callbacks & one for JavaScript)
    - Plain JavaScript `Function` types can be called with a simple `call0` function
