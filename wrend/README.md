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
-> Multiple ProgramIds per buffer / texture / framebuffer ?
    - Should HashMap's be nested by Program?
    - Buffer:
        - Only needs to be created & initialized ONCE 
        - Attribute location must be saved for each ProgramID
        - Ways to edit:
            - ATTRIBUTES can be adjusted to change how to pull data from buffers (unlikely)
            - New data can be uploaded to BUFFER (possible)
    ---> Create buffer THEN initialize attributes as separate links / processes
            

- Enable Vertex Array Object (VAO)

- Clean up buffer callback implementation (based off of uniform implementation)
    - Use separate callback structs

- Make passing in buffer update / should_update callbacks optional

- Return error when a duplicate item is added to HashMap?