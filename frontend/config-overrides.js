const webpack = require('webpack');

module.exports = function override(config, env) {
    // Add polyfills
    config.resolve.fallback = {
        ...config.resolve.fallback, // if you already have some fallbacks in your config
        "stream": require.resolve('stream-browserify'),
        "crypto": require.resolve('crypto-browserify'),
        "os": require.resolve('os-browserify/browser'),
        "path": require.resolve('path-browserify'),
        "zlib": require.resolve('zlib-browserify'),
        "url": require.resolve('url/'),
        "fs": false, // fs and http modules cannot be polyfilled for browser
        "http": false,
        "https": false,
        // fs and http modules cannot be polyfilled for browser
    };

    // Add the plugin
    config.plugins = [
        ...config.plugins,
        new webpack.ProvidePlugin({
            process: 'process/browser',
            Buffer: ['buffer', 'Buffer'],
        }),
    ];

    return config;
}
