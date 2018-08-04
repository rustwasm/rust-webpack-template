const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: "./js/index.js",
  output: {
    path: dist,
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
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new HtmlWebpackPlugin(),
  ]
};
