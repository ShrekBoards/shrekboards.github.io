const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    productionSourceMap: false,
    configureWebpack: {
        plugins: [
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, "."),
                withTypeScript: true
            }),
        ],
        module: {
            rules: [
                {
                    test: /\.wasm$/,
                    type: "webassembly/experimental"
                },
            ],
        },
    }
}