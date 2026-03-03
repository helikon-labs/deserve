#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
cd "$SCRIPT_DIR" || exit 1
cd ..
mkdir -p ./_chainspec

SKIP_PALLET_REVIVE_FIXTURES=1 cargo build --profile production -p deserve-node
# devnet
./target/production/deserve-node export-chain-spec \
    --chain=deserve_devnet \
    --output ./_chainspec/deserve-devnet-plain.json
# devnet raw
./target/production/deserve-node export-chain-spec \
    --raw \
    --chain=deserve_devnet \
    --output ./_chainspec/deserve-devnet-raw.json
# testnet
./target/production/deserve-node export-chain-spec \
    --chain=deserve_testnet \
    --output ./_chainspec/deserve-testnet-plain.json
# testnet raw
./target/production/deserve-node export-chain-spec \
    --raw \
    --chain=deserve_testnet \
    --output ./_chainspec/deserve-testnet-raw.json