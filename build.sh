#!/bin/bash
# Usage: ./build.sh [contract-name]
# Purpose: Builds a CosmWasm smart contract using the cosmwasm/optimizer Docker image.
# - Defaults to building the 'injective-cosmwasm-mock' contract if no argument is provided.
# - Specify a contract name (e.g., 'atomic-order-example') to build a different contract.
# - The contract must be a directory in 'contracts/' (e.g., 'contracts/[contract-name]').
# - Run from the workspace root (e.g., cw-injective) to ensure access to the workspace Cargo.toml.
# Examples:
#   ./build.sh                    # Builds injective-cosmwasm-mock
#   ./build.sh atomic-order-example  # Builds atomic-order-example
#   ./build.sh dummy              # Builds dummy
#   ./build.sh injective-cosmwasm-stargate-example  # Builds injective-cosmwasm-stargate-example
# Output: Produces an optimized .wasm file in target/wasm32-unknown-unknown/release/

ARCH=""

# Set architecture suffix for arm64
if [[ $(arch) = "arm64" ]]; then
  ARCH=-arm64
fi

# Default contract name
DEFAULT_CONTRACT="injective-cosmwasm-mock"
CONTRACT=${1:-$DEFAULT_CONTRACT}

# Validate that the contract directory exists
if [[ ! -d "contracts/$CONTRACT" ]]; then
  echo "Error: Contract directory 'contracts/$CONTRACT' does not exist."
  echo "Available contracts:"
  ls -1 contracts/
  exit 1
fi

# Run the optimizer with the specified or default contract
docker run --rm -v "$(pwd)":/code \
  -v "$HOME/.cargo/git":/usr/local/cargo/git \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer${ARCH}:0.16.1 /code/contracts/$CONTRACT






