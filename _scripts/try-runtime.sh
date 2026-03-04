#!/bin/bash
set -euo pipefail

cargo build --release --features try-runtime -p deserve-runtime
# see examples @ https://github.com/paritytech/try-runtime-cli?tab=readme-ov-file#examples
try-runtime \
    --runtime ./target/release/wbuild/deserve-runtime/deserve_runtime.wasm \
    on-runtime-upgrade \
    --blocktime 6000 \
    live --uri ws://127.0.0.1:7001 # set this to live RPC URL