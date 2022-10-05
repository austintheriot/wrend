# Contributing

There aren't any formal processes in place for contributing yet, as `wrend` is a very young project, but if you're interested in contributing, please feel free to reach out.

## Local dev environment

- Install the latest version of [nvm](https://github.com/nvm-sh/nvm) (for Linux / MacOS) or [nvm-windows](https://github.com/coreybutler/nvm-windows) (for Windows)
- Install node through `nvm` or `nvm-windows` (see .nvmrc for compatible Node version)
  - This should automatically install a compatible version of `npm` at the same time
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Install [Rust](https://www.rust-lang.org/tools/install)

## Prior to publishing a new version

- bump version manually in [`wrend/package.json`](wrend/package.json)
- bump version manually in [`wrend/Cargo.toml`](wrend/Cargo.toml)
- commit changes

## Publishing to npm

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

## Publishing to crates.io

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

## After publishing a new version

- update dependencies in /examples and /demos
- commit changes
