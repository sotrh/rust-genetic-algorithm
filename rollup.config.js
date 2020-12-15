import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: {
        example: "Cargo.toml",
    },
    output: {
        dir: "dist/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "js/"
        }),
    ],
};