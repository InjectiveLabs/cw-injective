# injective-protobuf

Rust protobuf files generation for Injective messages.

## Prerequisites

- `git submodule update --init`. Downloads the `sdk-go` repository, to reference the protobuf files under `./proto`.
- buf: `brew install bufbuild/buf/buf` on macOS or even Linux. See https://buf.build/docs/installation/ for detailed installation
    instructions.

## Usage

- `./scripts/protoexport.sh`

  Downloads / Updates Cosmos SDK protobuf files under `./third_party/proto/`, using `buf`.

- `cargo build`

  Generates the Rust protobuf files under `./src/proto`.

## Maintenance

- The specific version / tag of the Cosmos SDK proto files should be indicated in the `protoexport.sh` file.
- `git submodule update --remote` needs to be run, in order to update the local reference to `sdk-go`.
  For the release process, it would be important to define which branch / tag or commit in particular needs to be referenced.

This is something that must be checked as part of each major release, and the `sdk-go` reference updated accordingly in `dev`, before
tagging.

## Future work

Once the Injective protobuf files are stored in the Buf Schema Registry (BSR), and / or defined in gogo format,
`buf export` can be used to download them as well, instead of relying on git submodule.
