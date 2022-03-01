# Telecss (WIP)

CSS tokenizer, parser, transformer, written in Rust.

This project is mainly based on the following specs:

- [CSS Syntax Module Level 3](https://www.w3.org/TR/css-syntax-3) document.
- [Syntax and basic data types](https://www.w3.org/TR/CSS22/syndata.html#syntax) document.

# Benchmark

Benchmarks on each commit: [https://telecss.github.io/telecss/dev/bench/](https://telecss.github.io/telecss/dev/bench/)

This benchmark is based on parsing/tokenizing the `crates/telecss/examples/normalizecss/normalize.css` file, and the time-consuming of parsing includes the time-consuming of tokenizing:

- `Pure Parsing Time` = `Parsing Time` - `Tokenizing Time`

# TODO

## Current Focus

- [ ] Playground (with WASM)

## Schedule

- [x] Make tokenizer available.
- [x] Make parser available.
- [x] Benchmarks & CI (https://github.com/benchmark-action/github-action-benchmark)
- [x] Make transformer available.
- [x] Make generator available.
- [ ] Playground (with WASM)
- [ ] Allows writing plugins using Nodejs
- [ ] Generate code frame for diagnostics when errors occur.
- [ ] Docs
