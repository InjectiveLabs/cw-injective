# injective-protobuf

Rust protobuf files generation for Injective messages.

## Prerequisites

- `git submodule update --init`. Downloads the `sdk-go` repository, to reference the protobuf files under `./proto`.
- The files under `./third_party` need to be manually kept in sync with the ones from `injective-core` at the moment.

## Usage

`cargo build`

Generates the Rust protobuf files under `./src/proto`.
