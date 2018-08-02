const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: "./js/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js"
  },
  module: {
    rules: [
      {
        test: /Cargo.toml$/,
        loader: "@wasm-tool/rust-loader"
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin(),
  ]
};
