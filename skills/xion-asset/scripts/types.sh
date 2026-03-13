#!/bin/bash
set -e

# Asset Types - List available NFT asset types
# Usage: types.sh [--network testnet|mainnet]

NETWORK="${1:-testnet}"

xion-toolkit asset types --network "$NETWORK" --output json
