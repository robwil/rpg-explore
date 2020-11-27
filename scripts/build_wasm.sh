#!/bin/bash
set -euxo pipefail

cargo test
cargo clippy
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/rpg-explore.wasm .