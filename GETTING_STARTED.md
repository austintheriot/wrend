# Getting Started

- [Getting Started](#getting-started)
  - [Rust (compiling to WebAssembly)](#rust-compiling-to-webassembly)
  - [JavaScript / TypeScript](#javascript--typescript)
    - [npm](#npm)
    - [yarn](#yarn)
    - [Compatible JavaScript Bundlers](#compatible-javascript-bundlers)
    - [No Bundler Configuration](#no-bundler-configuration)
    - [Webpack Configuration](#webpack-configuration)
    - [Vite Configuration](#vite-configuration)

## Rust (compiling to WebAssembly)

See [crates.io package](https://crates.io/crates/wrend)

Add `wrend` as a dependency to your crate's Cargo.toml file:

```toml
# Add this to your project's Cargo.toml file
[dependencies]
wrend = "0.3.5"
```

## JavaScript / TypeScript

See [npm package](https://www.npmjs.com/package/wrend)

### npm

```sh
npm i wrend
```

### yarn

```sh
yarn add wrend
```

### Compatible JavaScript Bundlers

These bundlers are known to be compatible (implementation has been tested in the [examples directory](https://github.com/austintheriot/wrend/tree/master/examples)):

- No bundler: script fetched via CDN: [see configuration notes](#no-bundler-configuration)
- Webpack v5: [see configuration notes](#webpack-configuration)
- Vite: [see configuration notes](#vite-configuration)

These bundlers are likely to be compatible:

- Webpack v4
- Parcel 1 (supports wasm holistically, so likely to support `wrend`)

These bundlers are known to be incompatible:

- Parcel 2 (they do not yet support wasm holistically)

### No Bundler Configuration

```html
 <script type="module">
        // this is a CDN file that is auto-generated when `wrend` is published to npm
        import init, { Renderer, /* any other named imports go here*/ } from "https://cdn.jsdelivr.net/npm/wrend@0.3.5/wrend.js";

        const main = async () => {
            // it's necessary to initialize wasm module before using
            await init();

            // your rendering logic goes here
        }

        main();
    </script>
```

### Webpack Configuration

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

```js
// your webpack javascript entry file
import init, { Renderer, /* any other named imports go here*/ }  from 'wrend';

const main = async () => {
    // it's necessary to initialize wasm module before using
    await init();

    // your rendering logic goes here
}

main();
```

### Vite Configuration

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

```js
// your entry file for Vite
import init, { Renderer, /* any other named imports go here*/  } from 'wrend';

const main = async () => {
    // it's necessary to initialize wasm module before using
    await init();

    // your rendering logic goes here
}

main();
```
