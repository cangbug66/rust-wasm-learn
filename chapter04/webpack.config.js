const path = require('path');
module.exports = {
  entry: "./test.js",
  output: {
    path: path.resolve(__dirname, "web"),
    filename: "index.js",
  },
  mode: "development",
  devServer: {
    contentBase: path.join(__dirname, "web")
  }   
};