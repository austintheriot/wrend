const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
const devPath = path.resolve(__dirname, "pkg");
const staticFilesSrc = path.resolve(__dirname, "static");
const getStaticFilesOutputDir = (isProduction) => isProduction ? distPath : devPath;

module.exports = (env, argv) => {
  const isProduction = argv.mode === 'production';
  return {
    devServer: {
      port: 8000,
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
