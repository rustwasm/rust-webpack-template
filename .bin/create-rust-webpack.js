#!/usr/bin/env node

const { spawnSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const cpr = require("cpr");
const rimraf = require("rimraf");

function run(cmd, args, opts) {
  const output = spawnSync(cmd, args, opts);

  if (output.error != null) {
    throw output.error;
  }

  if (output.status !== 0) {
    throw new Error("Bad error code when running `" + cmd + " " + args.join(" ") + "`: " + output.status);
  }
}

let folderName = '.';

if (process.argv.length >= 3) {
  folderName = process.argv[2];
  if (!fs.existsSync(folderName)) {
    fs.mkdirSync(folderName);
  }
}

let gitFolder = path.join(folderName, "git-clone");

// This uses --no-tags and --depth 1 in order to make the cloning faster
run("git", ["clone", "--no-tags", "--depth", "1", "https://github.com/rustwasm/rust-webpack-template.git", gitFolder]);

// Copies the template folder
cpr(path.join(gitFolder, "template"), folderName, {}, function (err, files) {
  // Removes the git folder regardless of whether cpr succeeded or not
  rimraf.sync(gitFolder);

  if (err) {
    throw err;

  } else {
    console.log(" 🦀 Rust + 🕸 WebAssembly + Webpack = ❤️ ");

    run("npm", ["install"], { cwd: folderName, shell: true });

    console.log(" Installed dependencies ✅ ");
  }
});
