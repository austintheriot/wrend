# wrend

Note: **This library is currently experimental and not considered stable for public use**.

- [wrend](#wrend)
  - [About](#about)
  - [Why](#why)
  - [Demos](#demos)
    - [Ray Tracer](#ray-tracer)
    - [Particle Flow Field](#particle-flow-field)
    - [Conway's Game of Life](#conways-game-of-life)
    - [Larger Than Life](#larger-than-life)
    - [Scripts](#scripts)

## About

Wrend is a wrapper library around raw WebGL2 code and is written in Rust, which then gets compiled to WebAssembly for running in the browser. Its goal is to make working with WebGL & Rust together more convenient. Though most of the demo app examples are built using Yew, `wrend` itself is framework agnostic and is designed to be used in a variety of settings with diverse rendering pipelines.

If you're wondering about the name, `wrend` is short for **W**ebGL2 **Rend**ering Library.

## Why

This library exists because I found myself writing the same verbose, (occasionally `unsafe`) WebGL code over and over again, constantly struggling to find the right level and type of abstraction. Wrend is designed to ease the pain of working with low-level WebGL programming in Rust. One particular convenience Wrend provides is an abstraction over `requestAnimationFrame` calls, making continuous animations as simple as calling `start_animating` and then holding the returned handle in memory. Stopping is also as easy as dropping the handle and/or calling `stop_animating`.

Another strength of Wrend is its flexibility: rather than focusing on pure rasterization, Wrend enables constructing unique graphics pipelines for things like ray tracers, flow fields, and other non-traditional methods of rendering.

## Demos

To see a list of all interactive demos: [Demos](https://austintheriot.github.io/wrend/)

### [Ray Tracer](https://austintheriot.github.io/wrend/ray-tracer)

Realtime ray tracer written from scratch, inspired by the [Ray Tracing in One Weekend](https://raytracing.github.io/) series by Peter Shirley and adapted for use with Rust & WebGL.

I initially started this project as a software ray tracer running on Rust/WASM alone, but the render times that I experienced were so frustratingly slow that I quickly looked into implementing a hardware ray tracer that could take better advantage of the GPU's parallelization power. Once I switched to using WebGL2, render times went from around 1-6 minutes for a decent render to less than a second, and I was able to implement some realtime ray tracing elements like moving the camera, etc. by averaging many low-sample frames together rather than calculating them all at once.

![Ray-traced image created using the Wrend library](/demos/screenshots/ray_tracer.png)

### [Particle Flow Field](https://austintheriot.github.io/wrend/flow-field)

100,000 particles moving across the canvas, dropping pigment as they move. The movement of the particles is determined by a 2D noise texture generated at runtime.

![A particle flow field](/demos/screenshots/flow_field.png)

### [Conway's Game of Life](https://austintheriot.github.io/wrend/game-of-life)

The classic.

![Screenshot of Conway's Game of Life simulation](/demos/screenshots/game_of_life.png)

### [Larger Than Life](https://austintheriot.github.io/wrend/larger-than-life)

This is similar to the classic Conway's Game of Life, except it uses an 11x11 convolution kernel (rather than the classic 3x3) to calculate the next state of each cell. This results in more organic, formations that behave surprisingly similar to the original.

![Screenshot of the Larger Than Life simulation](/demos/screenshots/larger_than_life.png)

### Scripts

Publishing to npm:

```bash
# must be in `wrend` npm package directory
cd wrend

# builds library and outputs to /dist directory
npm run prepublish

cd dist

# must be logged into npm to publish
npm login

# publish package
npm publish
```

Publishing to crates.io:

```bash
# must be in `wrend` npm package directory
cd wrend

# do a dry run to make sure everything is compiling ook
cargo package

cargo publish
```
