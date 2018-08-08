#!/usr/bin/env node

const { spawn } = require("child_process");
const fs   = require("fs");

let folderName = '.';

if (process.argv.length >= 3) {
  folderName = process.argv[2];   
  if (!fs.existsSync(folderName)) {
    fs.mkdirSync(folderName);
  }
}

const clone = spawn("git", ["clone", "https://github.com/rustwasm/rust-webpack-template.git", folderName]);

clone.on("close", (code) => {
  if (code !== 0) {
    // TODO(sven): handle error here
    console.error()
    process.exit(code);
  } else {
    console.log("🦀Rust + 🕸 WebAssembly + Webpack = ❤️");
    // TODO(sven): npm install
  }
});
