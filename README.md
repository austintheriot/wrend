# wrend

**This library is currently in active development**. Feel free to use it, but do so with the knowledge that APIs are likely to change without consideration for backwards compatibility.

## Table of Contents

- [wrend](#wrend)
  - [Table of Contents](#table-of-contents)
  - [About](#about)
  - [Why](#why)
  - [Getting Started](#getting-started)
    - [Rust (compiling to WebAssembly)](#rust-compiling-to-webassembly)
    - [JavaScript / TypeScript](#javascript--typescript)
      - [npm](#npm)
      - [yarn](#yarn)
      - [Compatible JavaScript Bundlers](#compatible-javascript-bundlers)
      - [Webpack Configuration](#webpack-configuration)
      - [Vite Configuration](#vite-configuration)
  - [Documentation](#documentation)
  - [Examples](#examples)
  - [Demos](#demos)
    - [Ray Tracer](#ray-tracer)
    - [Particle Flow Field](#particle-flow-field)
    - [Conway's Game of Life](#conways-game-of-life)
    - [Larger Than Life](#larger-than-life)
  - [Contributing](#contributing)
    - [Local dev environment](#local-dev-environment)
    - [Publishing to npm](#publishing-to-npm)
    - [Publishing to crates.io](#publishing-to-cratesio)

## About

Wrend is a wrapper library around raw WebGL2 code and is written in Rust, which then gets compiled to WebAssembly for running in the browser. Its goal is to make working with WebGL more convenient when writing Rust and/or JavaScript/TypeScript code. Though most of the demo app examples are built using Yew, `wrend` itself is framework agnostic and is designed to be used in a variety of settings with diverse rendering pipelines. See the [examples directory](https://github.com/austintheriot/wrend/tree/master/examples) of the repo for examples.

If you're wondering about the name, `wrend` is short for **W**ebGL2 **Rend**ering Library.

## Why

This library exists because I found myself writing the same verbose, (occasionally `unsafe`) WebGL code over and over again, often struggling to find the right level and type of abstraction over WebGL calls. Wrend is designed to ease the pain of working with low-level WebGL programming in Rust. This includes abstraction over `requestAnimationFrame` calls, making continuous animations as simple as calling `start_animating` and then holding the returned handle in memory. Stopping is also as easy as dropping the returned `renderer` handle and/or calling `stop_animating`.

Another strength of Wrend is its flexibility: rather than focusing on more common 3D rasterization techniques, Wrend enables constructing unique graphics pipelines for things like ray tracers, flow fields, and other non-traditional methods of rendering.

## Getting Started

### Rust (compiling to WebAssembly)

See [crates.io package](https://crates.io/crates/wrend)

Add `wrend` as a dependency to your crate's Cargo.toml file:

```toml
# Add this to your project's Cargo.toml file
[dependencies]
wrend = "~0"
```

### JavaScript / TypeScript

See [npm package](https://www.npmjs.com/package/wrend)

#### npm

```sh
npm i wrend
```

#### yarn

```sh
yarn add wrend
```

#### Compatible JavaScript Bundlers

These bundlers are known to be compatible (implementation has been tested in the [examples directory](https://github.com/austintheriot/wrend/tree/master/examples)):

- Webpack v5: [see configuration notes](#webpack-configuration)
- Vite: [see configuration notes](#vite-configuration)

These bundlers are likely to be compatible:

- Webpack v4
- Parcel 1 (supports wasm holistically, so likely to support `wrend`)

These bundlers are known to be incompatible:

- Parcel 2 (they do not yet support wasm holistically)

#### Webpack Configuration

If you are using Webpack version 5, you must configure a few things in your `webpack.config.js` file:

```js
// webpack.config.js
module.exports = (env, argv) => {
  return {
    // ... your config items here

    // syncWebAssembly or syncWebAssembly must be enabled here
    experiments: {
      syncWebAssembly: true,
    },
  };
};

```

Then, when importing `wrend`, you must do so asynchronously:

```js
// import WebAssembly modules asynchronously
const { Renderer } = await import('wrend');
```

#### Vite Configuration

If you are using Vite as your bundler, you must configure a few things in your `vite.config.js` file:

```js
// vite.config.js
import wasm from "vite-plugin-wasm";

export default {
  optimizeDeps: {
    // must exclude `wrend` from Vite's automatic optimization 
    // to prevent weird initialization errors from the wasm module
    exclude: ['wrend']
  },
  plugins: [
    // this plugin is necessary to support npm modules that 
    // are generated with `wasm-pack`
    wasm(),
  ]
};
```

## Documentation

See latest documentation at [https://docs.rs/wrend/latest/wrend/](https://docs.rs/wrend/latest/wrend/)

## Examples

To see a list of simple, self-contained TypeScript and Rust projects that use `wrend`, see:

- [Examples directory](https://github.com/austintheriot/wrend/tree/master/examples) in the GitHub repo

## Demos

To see a list of more extensive demos that exhibit what is possible with the `wrend` library see:

- [Demos directory](https://github.com/austintheriot/wrend/tree/master/demos) in the GitHub repo
- [Live demos](https://austintheriot.github.io/wrend/) that you can interact with
- Continue reading below for more information on some of the featured demos

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

## Contributing

There aren't any formal processes in place for contributing yet, as `wrend` is a very young project, but if you're interested in contributing, please feel free to reach out.

### Local dev environment

- Install the latest version of [nvm](https://github.com/nvm-sh/nvm) (for Linux / MacOS) or [nvm-windows](https://github.com/coreybutler/nvm-windows) (for Windows)
- Install node through `nvm` or `nvm-windows` (see .nvmrc for compatible Node version)
  - This should automatically install a compatible version of `npm` at the same time
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Install [Rust](https://www.rust-lang.org/tools/install)

### Publishing to npm

```bash
# starting from the project root,
# must be in the actual npm package directory /wrend
cd wrend

# builds library and outputs to /dist directory
npm run prepublish

# publish happens from the /dist folder, 
# where built output files are located
cd dist

# must be logged into npm to publish
npm login

# publish package
npm publish
```

### Publishing to crates.io

```bash
# starting from the project root,
# must be in actual crate directory /wrend
cd wrend

# run tests before publishing
cargo test

# do a dry run to make sure everything is bundling ok
cargo package

cargo publish
```
