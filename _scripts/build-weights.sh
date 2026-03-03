#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
cd "$SCRIPT_DIR" || exit 1
cd ..

SKIP_PALLET_REVIVE_FIXTURES=1 cargo build --profile production -p deserve-node --features runtime-benchmarks
./target/production/deserve-node benchmark pallet \
    --chain=deserve_testnet \
    --wasm-execution=compiled \
    --pallet=pallet_deserve \
    --extrinsic=* \
    --steps=50 \
    --repeat=20 \
    --template=./_scripts/frame-weight-template.hbs \
    --output ./pallets/deserve/src/weights.rs
