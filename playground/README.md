# How to build WASM

```
rustup target add wasm32-unknown-unknown
cargo build -p telecss --target wasm32-unknown-unknown --lib

# Ensure that the cli version is the same as the bindgen version used in telecss crate.
cargo install -f wasm-bindgen-cli --version 0.2.79

mkdir wasm
wasm-bindgen ../target/wasm32-unknown-unknown/debug/telecss.wasm --target web --out-dir ./wasm/
```
