# Wrend

The intent for this crate is to serve as a reusable, generic utility crate for doing WebGl rendering.

## Use Cases

The following use cases should guide the development of this library:

- The ability to save uniform locations and update all uniforms programmatically.
- Perform transform feedback, where all data stays on the GPU
  - Particle systems
- Render to a framebuffer and then retrieve previous render for sampling
  - Conway's Game of Life
  - Any iterative kernel-based rendering
- Ray tracing

## Todo

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
