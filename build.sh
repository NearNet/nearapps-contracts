#!/usr/bin/env bash

set -x

RUSTFLAGS='-C linker=clang-9' cargo build --target wasm32-unknown-unknown --release
