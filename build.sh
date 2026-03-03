#!/bin/bash
# Usage: ./build.sh <alias|contract-dir>
# Aliases:
#   stargate  -> injective-cosmwasm-stargate-example
#   mock      -> injective-cosmwasm-mock
#   atomic    -> atomic-order-example
#   dummy     -> dummy
# Full contract directory names also accepted.

ARCH=""
if [ "$(arch)" = "arm64" ]; then
  ARCH=-arm64
fi

resolve_alias() {
  case "$1" in
    stargate) echo "injective-cosmwasm-stargate-example" ;;
    mock)     echo "injective-cosmwasm-mock" ;;
    atomic)   echo "atomic-order-example" ;;
    dummy)    echo "dummy" ;;
    *)        echo "$1" ;;
  esac
}

if [ -z "$1" ]; then
  echo "Usage: ./build.sh <alias|contract-dir>"
  echo ""
  echo "Aliases:"
  echo "  stargate  -> injective-cosmwasm-stargate-example"
  echo "  mock      -> injective-cosmwasm-mock"
  echo "  atomic    -> atomic-order-example"
  echo "  dummy     -> dummy"
  echo ""
  echo "Available contract directories:"
  ls -1 contracts/
  exit 1
fi

CONTRACT=$(resolve_alias "$1")

if [ ! -d "contracts/$CONTRACT" ]; then
  echo "Error: Contract directory 'contracts/$CONTRACT' does not exist."
  echo "Available contracts:"
  ls -1 contracts/
  exit 1
fi

docker run --rm -v "$(pwd)":/code \
  -v "$HOME/.cargo/git":/usr/local/cargo/git \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer${ARCH}:0.17.0 /code/contracts/$CONTRACT






