#!/bin/sh
cargo build --features reqwest
cargo build --features spin --target wasm32-wasi
cargo build --features esp32 --target riscv32imc-esp-espidf  -Zbuild-std=std,panic_abort

