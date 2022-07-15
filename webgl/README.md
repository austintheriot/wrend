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

## Todo
- Enable Vertex Array Object (VAO)

- Clean up buffer callback implementation (based off of uniform implementation)
    - Use separate callback structs


- In the micro apps, use style.scss from /entry

- Rename webgl library to wrend

- Enable other lifetime callbacks for uniforms (create, should update, update)

- Animations:
    - Move animation logic OUTSIDE of Renderer, so that the animation callback has access to the renderer context?

- Start / stop animation
    - Implement Drop for Renderer such that the animation frame gets canceled when the object is dropped
    - Check if should be animating before calling animation callback

- Create framebuffer
- Create texture

- Add an optional callback to see if uniform should update
- Make passing in buffer update / should_update callbacks optional

- Return error when a duplicate item is added to HashMap