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

- If `Renderer` MUST get cloned on each iteration: consider wrapping internal data in Rc<RefCell>
- OR make a note that the operation could be expensive
- -----------------> Wrap RendererJs data in an Rc<RefCell<_>> so that clones to the data are inexpensive?

- Use static methods to render in JavaScript, rather than downcasting from an `Any`

- Make it more explicit when structs get cloned to convert them into a `JsValue`

- use `dyn_ref` for Renderer ?

- Call `initialize recorder` automatically if it has not been initialized before `start_recording` is called

- Make a clean wrapper around `Either` that functions as a callback abstraction (instead of implementing function utilities on `Either` itself)

- Add custom TypeScript types for *_create_context.rs callbacks

- Make it possible access `Renderer` from the render callback in JavaScript

- Implementing JavaScript API
  - Check that all structs implement `pub fn new()` where appropriate
  - Check that all structs have `#[wasm_bindgen(constructor)]` where needed
  - Make sure all public JavaScript API struct docs point / link to their respective Rust counterparts for documentation
  - Derive all traits for Js structs
  - Update pretty much all Js functions to receive and return `self`

- Make a procedural macro and/or a custom derive macro for `Id`, and possible `IdName`

- Make some impl that do not need to be called from outside pub(crate)
  - Especially the `new` functions (or the structs themselves?) of Buffer, Uniform, etc. (internal types not built directly)

- Move `js_conversion` utils into a generic trait impl on a new type around Vec

- Bug: WebGL2 rendering contexts are not getting discarded when handle is dropped: enable this manually through browser extension (see MDN WebGL best practices article)

- Only set up RecordingData when requested

- Do not use dynamic functions for callbacks

- Enable recording video from canvas
  - Enable starting/stopping recording and downloading as separate steps

- Add event system for emitting events related to changes of internal state

- Clean up "into" ergonomics around animation_handle and recording_handle

- Fix callback system (model after Yew's `Callback` type--using generic instead of dynamic dispatch)

- Make using uniform links more ergonomic: use builder pattern, etc.

- Make a trait for Texture numbers that is available from the Renderer?

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
  - Make it renderer agnostic?
  - Make recording codec customizable

- Make cross-compatible /callable with vanilla JavaScript
  - This would require making sure all returned handles can cross the WasmAbi
  - It would also probably require a refactor of callbacks, so that plain JavaScript closures could be accepted as well
    - This could be achieved with a enum for callbacks (one for plain Rust callbacks & one for JavaScript)
    - Plain JavaScript `Function` types can be called with a simple `call0` function
