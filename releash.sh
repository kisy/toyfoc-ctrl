#!/usr/bin/env bash
cargo build --release && espflash flash target/riscv32imc-esp-espidf/release/toyfoc-ctrl
