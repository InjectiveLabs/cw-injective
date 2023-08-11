# injective-protobuf

Rust protobuf files generation for Injective messages.

## Prerequisites

- buf: `brew install bufbuild/buf/buf` on macOS or even Linux. See https://buf.build/docs/installation/ for detailed installation
    instructions.

## Usage

- `./scripts/protoexport.sh`

  Downloads / Updates Injective and Cosmos SDK protobuf files under `./proto` and `./third_party/proto/`, using `buf`.

- `cargo build`

  Generates the Rust protobuf files under `./src/proto`.

## Maintenance

- The specific version / tag of the Injective and Cosmos SDK proto files should be indicated in the `protoexport.sh` file.

This is something that must be checked as part of each major release, and the `sdk-go` and `cosmos-sdk` reference updated accordingly.
