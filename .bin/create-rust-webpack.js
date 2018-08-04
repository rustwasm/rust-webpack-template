#!/usr/bin/env node

const { spawn } = require("child_process");

const clone = spawn("git", ["clone", "https://github.com/rustwasm/rust-webpack-template.git", "."]);

clone.on("close", (code) => {
  if (code !== 0) {
    // TODO(sven): handle error here
    console.error()
    process.exit(code);
  } else {
    console.log("Rust + WebAssembly + Webpack = <3");
    // TODO(sven): npm install
  }
});
