const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const WebpackShellPluginNext = require('webpack-shell-plugin-next');
const childProcess = require("child_process")
const cpy = require("cpy")

module.exports = {
    entry: './index.ts',
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "index.html"
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "./wasm_export")
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        }),
        new WebpackShellPluginNext({
            onBuildStart: {
                scripts: [
                    async () => {
                        let fittingEngineGen = path.resolve(__dirname, "../fitting_engine_ts_gen")
                        let stdout = await new Promise((resolve, reject) => {
                            childProcess.exec("cargo test", {
                                cwd: fittingEngineGen
                            }, (err, stdout, stderr) => {
                                if (err) {
                                    reject(err)
                                } else {
                                    resolve(stdout)
                                }
                            })
                        })
                        await cpy([path.resolve(fittingEngineGen, "bindings") + "/*"], "gen/types/")
                    }
                ],
                blocking: false,
                parallel: true,
            },
            afterDone: {scripts: [], blocking: false, parallel: false}
        })
    ],
    mode: 'development'
};
