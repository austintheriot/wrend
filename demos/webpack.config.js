const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
const devPath = path.resolve(__dirname, "pkg");
const staticFilesSrc = path.resolve(__dirname, "static");

module.exports = (env, argv) => {
  const isProduction = argv.mode === 'production';
  return {
    devServer: {
      static: {
        directory:  isProduction ? distPath : devPath,
      },
      compress: isProduction,
      port: 8000,
    },
    experiments: {
      syncWebAssembly: true,
    },
    entry: './index.js',
    output: {
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
        patterns: [
          ...(
            isProduction ? [
              { from: staticFilesSrc, to: distPath },
            ] : [
              { from: staticFilesSrc, to: devPath },
            ])
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
      }),
      new CleanWebpackPlugin(),
    ],
    watch: argv.mode !== 'production'
  };
};
