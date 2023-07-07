# injective-protobuf

Rust protobuf files generation for Injective messages.

## Prerequisites

- `git submodule update --init`. Downloads the `sdk-go` repository, to reference the protobuf files under `./proto`.
- The files under `./third_party` need to be manually kept in sync with the ones from `injective-core` at the moment.

## Usage

`cargo build`

Generates the Rust protobuf files under `./src/proto`.

## Maintenance

- `git submodule update --remote` needs to be run in order to update the local reference to `sdk-go`.
  For the release process, it would be important to define which branch / tag or commit in particular needs to be referenced.
  This is something that must be checked as part of each major release, and the `sdk-go` reference updated accordingly in `dev`, before
  tagging.
