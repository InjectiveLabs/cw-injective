#!/bin/bash
ARCH=""

if [[ $(arch) = "arm64" ]]; then
  ARCH=-arm64
fi

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

docker run --rm -v "$WORKSPACE_ROOT":/code -v "$HOME/.cargo/git":/usr/local/cargo/git \
  --mount type=volume,source="$(basename "$WORKSPACE_ROOT")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer${ARCH}:0.17.0 "./"
