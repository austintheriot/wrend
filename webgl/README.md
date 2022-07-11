# WebGL Utility Library

The intent for this crate is to serve as a reusable, generic utility crate for doing WebGl rendering.

## Use Cases

The following use cases should guide the development of this library:

- The ability to save uniform locations and update all uniforms programmatically.
- Perform transform feedback, where all data stays on the GPU
    - Particle systems
- Render to a framebuffer and then retrieve previous render for sampling
    - Conway's Game of Life
    - Any iterative kernel-based rendering
- Ray tracing?