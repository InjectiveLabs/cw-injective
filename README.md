# Injective Cosmwasm Contracts

This repository contains the full source code for the core smart contracts deployed
on [Injective Protocol](https://injective.com).

## Contracts

These contracts hold the core logic of the system.

| Contract                            | Description                                            |
|-------------------------------------|--------------------------------------------------------|
| [`registry`](./contracts/registry/) | Logic for the BeginBlocker contract execution registry |

## Development

### Environment Setup

- Rust v1.57.0
- `wasm32-unknown-unknown` target
- Docker

1. Install [`rustup`](https://rustup.rs)
2. Run the following

```shell
rustup default 1.57.0
rustup target add wasm32-unknown-unknown
```

3. Make sure [Docker](https://docker.com) is installed on your machine

### Unit / Integration Test

Each contract contains Rust unit tests embedded within the contract source directories. You can run

```shell
cargo unit-test
```

### Compiling

Go to the contract directory and run

After making sure tests pass, you can compile each contract with the following

```bash
RUSTFLAGS='-C link-arg=-s' cargo wasm
sha256sum target/wasm32-unknown-unknown/release/<CONTRACT_NAME>.wasm
```

#### Production

For production builds, run the following:

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

or

```bash
chmod +x build_release.sh
./build_release.sh
```

This performs several optimizations which can significantly reduce the final size of the contract binaries, which will
be available inside the `artifacts/` directory.

## Formatting

Make sure you run `rustfmt` before creating a PR to the repo. You need to install the `nightly` version of `rustfmt`.

```
rustup toolchain install nightly
```

To run `rustfmt`,

```
cargo fmt
```

## Linting

You should run `clippy` also. This is a lint tool for Rust. It suggests more efficient/readable code. You can
see [the clippy document](https://rust-lang.github.io/rust-clippy/master/index.html) for more information. You need to
install `nightly` version of `clippy`.

### Install

```
rustup toolchain install nightly
```

### Run

```
cargo clippy -- -D warnings
```

## Testing

Developers are strongly encouraged to write unit tests for new code, and to submit new unit tests for old code. Unit
tests can be compiled and run with: `cargo test --all`. For more details, please reference Unit Tests.

[//]: # (## Bug Bounty)

[//]: # (The contracts in this repo are included in a bug bounty program)
