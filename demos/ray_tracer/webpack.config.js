const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyPlugin = require("copy-webpack-plugin");

const distPath = path.resolve(__dirname, "dist");
const staticFilesSrc = path.resolve(__dirname, "static");

module.exports = (env, argv) => {
  const isProduction = argv.mode === 'production';
  return {
    devServer: {
      port: 8000,
      static: {
        directory: distPath,
      },
      historyApiFallback: {
        index: '/'
      },
      open: true,
    },
    experiments: {
      syncWebAssembly: true,
    },
    entry: './index.js',
    output: {
      path: distPath,
      filename: "main.js",
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
      new CopyPlugin({
        patterns: [{
          from: staticFilesSrc, to: distPath
        }]
      }),

      new WasmPackPlugin({
        crateDirectory: __dirname,
        forceMode: isProduction ? "production" : "development",
      }),
    ],
    mode: isProduction ? "production" : "development",
  };
};
