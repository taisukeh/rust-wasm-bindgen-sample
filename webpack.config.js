const path = require('path');

module.exports = {
    resolve: {
        modules: [
            path.resolve('./wasm/pkg'),
            "node_modules",
        ],
    },
    output: {
        publicPath: '/dist/',
    },
    devServer: {
        contentBase: './public',
        watchContentBase: true
    },
};
