type Env = {
  WASM_FILE_PATH: string;
  JS_FILE_PATH: string;
};

export const env: Env = {
  WASM_FILE_PATH:
    "rust/deno-wasm/target/wasm32-unknown-unknown/release/deno_wasm.wasm",
  JS_FILE_PATH: "rust/deno-wasm/pkg/deno_wasm.js",
};
