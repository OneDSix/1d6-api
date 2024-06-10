const path = require('path');
const webpack = require('webpack')
const NodePolyfillPlugin = require('node-polyfill-webpack-plugin');

module.exports = {
  entry: './src/index.ts',
  target: 'web',
  mode: 'development',
  plugins: [
    new NodePolyfillPlugin(),
    new webpack.ProvidePlugin({
      fetch: 'node-fetch',
      crypto: 'crypto-browserify',
      resourceRegExp: /^node:/,
    }),
  ],
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    extensions: ['.ts', '.js'],
    fallback: {
      "crypto": require.resolve("crypto-browserify"),
      "stream": require.resolve("stream-browserify"),
      "buffer": require.resolve("buffer/"),

    }
  },
  output: {
    filename: 'bundle.web.js',
    path: path.resolve(__dirname, 'dist'),
    library: 'ApiSdk',
    libraryTarget: 'umd',
    globalObject: 'this',
  }
};
