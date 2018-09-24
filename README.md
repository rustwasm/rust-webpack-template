# `rust-webpack-template`

**Kickstart your Rust, WebAssembly, and Webpack project!**

This template comes pre-configured with all the boilerplate for compiling Rust
to WebAssembly and hooking into a Webpack build pipeline.

* `npm run start` -- Serve the project locally for
  development at `http://localhost:8080`.

* `npm run build` -- Bundle the project (in production mode)

_Note: This project assumes that you have installed [`wasm-pack`](https://github.com/rustwasm/wasm-pack). Since it is needed for webpack-plugin to work._

## Using This Template

```sh
cargo install wasm-pack
```

```sh
npm init rust-webpack my-app
```
