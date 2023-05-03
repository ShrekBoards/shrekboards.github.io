const path = require("path")
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    productionSourceMap: false,
    runtimeCompiler: true,
    configureWebpack: {
        experiments: {
            syncWebAssembly: true
        },
        plugins: [
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, "./src/wasm"),
                withTypeScript: true
            }),
        ],
        module: {
            rules: [
                {
                    test: /\.wasm$/,
                    type: "webassembly/sync",
                },
            ],
        },
        resolve: {
            extensions: [".tsx", ".ts", ".vue", ".wasm"]
        }
    }
}