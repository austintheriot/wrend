const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
const devPath = path.resolve(__dirname, "pkg");
const staticFilesSrc = path.resolve(__dirname, "static");
const getStaticFilesOutputDir = (isProduction) => isProduction ? distPath : devPath;

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
        directory: getStaticFilesOutputDir(isProduction),
      },
      historyApiFallback: {
        index: REPO_SLUG
      },
    },
    experiments: {
      syncWebAssembly: true,
    },
    entry: './index.js',
    ignoreWarnings: [
      // suppress all webpack compile warnings
      (warning) => true,
    ],
    output: {
      // this makes URLs compatible with GitHub pages, which is 
      // hosted at this project's repo name currently
      publicPath: REPO_SLUG,
      // this is local path to output to
      path: distPath,
      filename: "main.js",
      webassemblyModuleFilename: "main.wasm",
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: [
            'style-loader',
            'css-loader',
            'sass-loader',
          ],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [{
          from: staticFilesSrc, to: getStaticFilesOutputDir(isProduction)
        }],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
      }),
      new CleanWebpackPlugin(),
    ],
    watch: argv.mode !== 'production'
  };
};
