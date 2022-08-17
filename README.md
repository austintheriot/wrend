# wrend

a.k.a **W**ebGL2 **Rend**ering Library

Note: **This library is currently experimental and not considered stable for public use**.

## About

Wrend is a wrapper library around raw WebGL2 code and is written in Rust, which is compiled to WebAssembly for running in the browser. Its goal is to make working with WebGL & Rust together in the browser more convenient.

## Why

This library exists because I found myself writing the same verbose, (occasionally `unsafe`) WebGL code over and over again, constantly struggling to find the right level and type of abstraction. Wrend is designed to ease the pain of working with low-level WebGL programming easier in Rust. One particular convenience Wrend provides is an abstraction over `requestAnimationFrame` calls, making continuous animations as simple as calling start and holding the returned handle in memory. Stopping is also as easy as dropping the handle and/or calling stop.

Another strength of Wrend is its flexibility: rather than focusing on pure rasterization, Wrend enables the ability to construct very unique pipelines for things like ray tracers, flow fields, and other non-traditional methods of rendering.

## Demos

See a list a list of all interactive demos: [Demos](https://austintheriot.github.io/wrend/)

All of the following demos were created using the `wrend` library.


### [Ray Tracer](https://austintheriot.github.io/wrend/ray-tracer)

![Ray-traced image created using the Wrend library](/demos/screenshots/ray_tracer.png)

### [Flow Field](https://austintheriot.github.io/wrend/flow-field)

![A particle flow field](/demos/screenshots/flow_field.png)

### [Conway's Game of Life](https://austintheriot.github.io/wrend/game-of-life)

![Screenshot of Conway's Game of Life simulation](/demos/screenshots/game_of_life.png)

### [Larger Than Life](https://austintheriot.github.io/wrend/larger-than-life)

![Screenshot of the Larger Than Life simulation](/demos/screenshots/larger_than_life.png)
