#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
cd "$SCRIPT_DIR" || exit 1
cd ..
mkdir -p ./_deployment

SKIP_PALLET_REVIVE_FIXTURES=1 cargo build --profile production -p deserve-node
./target/production/deserve-node export-genesis-wasm \
    --chain=deserve_testnet \
    ./_deployment/deserve-testnet.wasm
./target/production/deserve-node export-genesis-wasm \
    --raw \
    --chain=deserve_testnet \
    ./_deployment/deserve-testnet-raw.wasm
./target/production/deserve-node export-genesis-head \
    --chain=deserve_testnet \
    ./_deployment/deserve-testnet.head
./target/production/deserve-node export-genesis-head \
    --raw \
    --chain=deserve_testnet \
    ./_deployment/deserve-testnet-raw.head
