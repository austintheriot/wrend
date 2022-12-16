# wrend

[![Crates Version]][crates.io] [![NPM Version]][npm] [![CI Image]][wrend CI] [![Docs Image]][docs.rs]

[Crates Version]: https://img.shields.io/crates/v/wrend.svg
[NPM Version]: https://img.shields.io/npm/v/wrend.svg
[Docs Image]: https://img.shields.io/badge/docs.rs-wrend-blue
[CI Image]: https://img.shields.io/github/actions/workflow/status/austintheriot/wrend/ci.yml?branch=master

[crates.io]: https://crates.io/crates/wrend
[npm]: https://www.npmjs.com/package/wrend
[docs.rs]: https://docs.rs/wrend/latest/wrend/
[wrend CI]: https://github.com/austintheriot/wrend/actions/workflows/ci.yml

**This library is currently in early development**. Feel free to use it, but do so with the knowledge that APIs are likely to change without consideration for backwards compatibility.

## Quick Links

- [Documentation][docs.rs]
- [Getting Started](GETTING_STARTED.md)
- [Simple Demo Apps (code)](examples)
- [Extensive Demo Apps (code)](demos)
- [Extensive Demo Apps (live)](https://austintheriot.github.io/wrend/)
- [Contributing](CONTRIBUTING.md)

## About

Wrend is a wrapper library around raw WebGL2 code and is written in Rust, which then gets compiled to WebAssembly for running in the browser. Its goal is to make working with WebGL more convenient when writing Rust and/or JavaScript/TypeScript code. Though most of the demo app examples are built using Yew, `wrend` itself is framework agnostic and is designed to be used in a variety of settings with diverse rendering pipelines. See the [examples directory](examples) of the repo for examples.

If you're wondering about the name, `wrend` is short for **W**ebGL2 **Rend**ering Library.

## Why

This library exists because I found myself writing the same verbose, (occasionally `unsafe`) WebGL code over and over again, often struggling to find the right level and type of abstraction over WebGL calls. Wrend is designed to ease the pain of working with low-level WebGL programming in Rust. This includes abstraction over `requestAnimationFrame` calls, making continuous animations as simple as calling `start_animating` and then holding the returned handle in memory. Stopping is also as easy as dropping the returned `renderer` handle and/or calling `stop_animating`.

Another strength of Wrend is its flexibility: rather than focusing on more common 3D rasterization techniques, Wrend enables constructing unique graphics pipelines for things like ray tracers, flow fields, and other non-traditional methods of rendering.

## Demos

### [Ray Tracer](https://austintheriot.github.io/wrend/ray-tracer)

Realtime ray tracer written from scratch, inspired by the [Ray Tracing in One Weekend](https://raytracing.github.io/) series by Peter Shirley and adapted for use with Rust & WebGL.

I initially started this project as a software ray tracer running on Rust/WASM alone, but the render times that I experienced were so frustratingly slow that I quickly looked into implementing a hardware ray tracer that could take better advantage of the GPU's parallelization power. Once I switched to using WebGL2, render times went from around 1-6 minutes for a decent render to less than a second, and I was able to implement some realtime ray tracing elements like moving the camera, etc. by averaging many low-sample frames together rather than calculating them all at once (progressive rendering).

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
