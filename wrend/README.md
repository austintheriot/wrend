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

- For every program, create an associated VAO
- Create all buffers
- Create all attributes
- Each attribute specifies which Program/VAO pair it should be linked to
- Save Attribute's Attribute position for all Programs it is associated with
- When the attribute is configured, update all VAOs that are linked to the program it corresponds to, 
    using the Attribute's AttributePosition that was saved for that Program/VAO
- When switching programs, simultaneously switch the VAO so that previous attribute configuration still works

- Make a trait for Texture numbers 

- Question: 
    - Updating how attributes are configured: should be updated for Programs/VAOs simultaneously
    - Updating buffers just means uploading new data, which is a global operation for all consuming Programs/VAOs

- Cleanup:
    - Consume links when they are used during build time - would require less cloning in general
    - Enable borrowing in context structs - would also require less cloning to occur

- Road map:
    - Add Vec / Matrix utilities
    - Enable compiling new programs at run time

- Enable transform feedback
            
- Make passing in buffer update / should_update callbacks optional

- Return error when a duplicate item is added to HashMap?