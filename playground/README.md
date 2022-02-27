# How to build WASM

```sh
rustup target add wasm32-unknown-unknown
cd crates/telecss
wasm-pack build --out-dir ../../playground/wasm --target web
```
