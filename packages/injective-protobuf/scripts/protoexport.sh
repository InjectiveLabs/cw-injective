#!/usr/bin/env bash
set -eo pipefail

echo "Exporting gogo proto files"
echo "Injective"
buf export --exclude-imports https://github.com/InjectiveLabs/sdk-go.git --output ./
# Remove spurious extra exported dir
rm -rf ./exchange
echo "Cosmos"
buf export https://github.com/cosmos/cosmos-sdk.git --output ./third_party/proto
