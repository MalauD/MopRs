const path = require('path');

module.exports = {
  entry: "./static/Js/index.jsx",
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: ["babel-loader"],
      },
    ],
  },
  resolve: {
    extensions: ["*", ".js", ".jsx"],
  },
  output: {
    filename: '[name].bundle.js',
    path: path.resolve(__dirname, 'static/Dist'),
    clean: true,
  },
};
