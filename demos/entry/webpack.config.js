const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const distPath = path.resolve(__dirname, "dist");
const staticFilesSrc = path.resolve(__dirname, "static");

// there is a lot of "magic" happening in the index.html and 404.html files 
// to make this repo compatible as a SPA with GitHub pages.
// 
// See https://github.com/rafgraph/spa-github-pages for detailed instructions
//
// This config deviates from the instructions slightly in the dev server setup, 
// which is probably necessary because the inclusion of a `<base />` tag
// in the index.html. This allows us not to have to specify the base url for every route
// in our Yew app
const REPO_SLUG = '/wrend/';

module.exports = (env, argv) => {
  const isProduction = argv.mode === 'production';
  return {
    devServer: {
      port: 8000,
      static: {
        directory: distPath,
      },
      historyApiFallback: {
        index: REPO_SLUG
      },
      open: ['/wrend'],
    },
    experiments: {
      syncWebAssembly: true,
    },
    entry: "./js/index.js",
    output: {
      // this makes URLs compatible with GitHub pages, which is 
      // hosted at this project's repo name currently
      publicPath: REPO_SLUG,
      // this is local path to output to
      path: distPath,
      filename: "main.js",
    },
    plugins: [
      new CopyPlugin({
        patterns: [{
          from: staticFilesSrc, to: distPath
        }]
      }),

      new WasmPackPlugin({
        crateDirectory: __dirname,
      }),
    ],
    mode: isProduction ? "production" : "development",
    watchOptions: {
      ignored: /node_modules/,
    },
  }
};
