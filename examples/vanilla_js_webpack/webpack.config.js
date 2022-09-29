const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");

const distPath = path.resolve(__dirname, "dist");
const staticFilesSrc = path.resolve(__dirname, "static");

module.exports = (env, argv) => {
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
    entry: './index.js',
    output: {
      path: distPath,
      filename: "main.js",
    },
    experiments: {
      syncWebAssembly: true,
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
        {
          test: /\.(glsl|frag|vert)$/i,
          type: 'asset/source',
        },
      ],
    },
    plugins: [
      new CopyPlugin({
        patterns: [{
          from: staticFilesSrc, to: distPath
        }]
      }),
    ],
  };
};
