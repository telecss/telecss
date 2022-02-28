# How to build WASM

```sh
rustup target add wasm32-unknown-unknown
cd crates/tele_wasm
wasm-pack build --out-dir wasm --target web
```
