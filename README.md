<div align="center">

  <h1><code>rust-webpack-template</code></h1>

  <strong>Kickstart your Rust, WebAssembly, and Webpack project!</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/create-wasm-app"><img src="https://img.shields.io/travis/rustwasm/create-wasm-app.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/hybrid-applications-with-webpack/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

This template is designed for creating monorepo-style Web applications with
Rust-generated WebAssembly and Webpack without publishing your wasm to NPM.

[**📚 Read this template's tutorial! 📚**][template-docs]

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/hybrid-applications-with-webpack/index.html

## 🚴 Using This Template

You can use `npm init` to clone this template:

```sh
npm init rust-webpack my-app
```

[Afterwards check out the full documentation for exploring it][template-docs].

## 🔋 Batteries Included

This template comes pre-configured with all the boilerplate for compiling Rust
to WebAssembly and hooking into a Webpack build pipeline.

* `npm start` -- Serve the project locally for development at
  `http://localhost:8080`. It auto-reloads when you make any changes.

* `npm run build` -- Bundle the project (in production mode).

* `npm test` -- Run the project's unit tests.
