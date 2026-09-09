const path = require('path');

module.exports = {
  entry: {
    udp: './src/udp.js',
  },
  output: {
    filename: '[name].bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
};