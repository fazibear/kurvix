OUT := "./out"
WASM_FILE := "./target/wasm32-unknown-unknown/release/kurvix.wasm"

wasm_setup:
  @cargo install wasm-bindgen-cli
  @rustup target install wasm32-unknown-unknown

wasm_build: wasm_setup
  @cargo build --target wasm32-unknown-unknown --release

wasm_bindgen: wasm_build
  @wasm-bindgen --no-typescript --target web --out-dir {{OUT}} {{WASM_FILE}}

wasm_opt: wasm_bindgen
  @wasm-opt -Os {{OUT}}/kurvix_bg.wasm -o {{OUT}}/kurvix_bg.wasm

wasm_copy: wasm_bindgen
  @cp -v index.html {{OUT}}
  @cp -v -r assets {{OUT}}

wasm: wasm_copy
