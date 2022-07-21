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

- Rethink Wrend's default behavior around VAOs:
    - Make them end-user configurable--to prevent the issue with transform feedback, where the default behavior just does not work
    - ----> Supply VAO ids (instead of ProgramIds) for each AttributeLInk and then provide a utility function for binding the desired VAO during render time
        - This would decouple the Program and/or the buffers from what the user wants to specify in the VAOs 
        - In the majority of cases, the same ProgramId would suffice, but in the case of transform feedback, for example, 
        it would greatly simplify things to be able to specify custom VAO bindings to attributes

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