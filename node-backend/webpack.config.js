const path = require('path');
const nodeExternals = require('webpack-node-externals');
const NodemonPlugin = require('nodemon-webpack-plugin');

const { NODE_ENV = 'production' } = process.env;

const devPlugins = NODE_ENV === 'development' ? [new NodemonPlugin()] : [];

module.exports = {
  entry: './src/index.ts',
  externals: [nodeExternals()],
  mode: NODE_ENV,
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: ['ts-loader'],
      },
    ],
  },
  output: {
    filename: 'index.js',
    path: path.resolve(__dirname, 'dist'),
  },
  plugins: [...devPlugins],
  resolve: {
    extensions: ['.ts', '.js'],
  },
  target: 'node',
  watch: NODE_ENV === 'development',
};
