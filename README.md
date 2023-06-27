# Injective Cosmwasm

This repository contains packages and examples for how to use CosmWasm with [Injective](https://injective.com).

## Packages

These packages can be used to integrate CosmWasm and Injective.

| Contract                                                          | Description                                                                                                                                                                      |
| ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`injective-cosmwasm`](./packages/injective-cosmwasm/)            | Package for Injective-specific queries and messages.                                                                                                                             |
| [`injective-math`](./packages/injective-math/)                    | Math utility library including custom FPDecimal type.                                                                                                                            |
| [`injective-protobuf`](./packages/injective-protobuf/)            | Rust protobuf files generation for Injective messages.                                                                                                                           |
| [`injective-std`](./packages/injective-std/)                      | Injective's proto-generated types and helpers built using [Buf](https://github.com/bufbuild/buf). Enables interaction with both custom and standard modules.                     |

## Example Contracts

These contracts can be used as examples for CosmWasm and Injective.

| Contract                                                          | Description                                                                                 |
| ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| [`dummy`](./contracts/dummy/)                                     | A simply template for starting a new contract.                                              |
| [`atomic-order-example`](./contracts/atomic-order-example/)       | Example contract on how to do atomic market orders on Injective incl handling the response. |
| [`swap-contract`](https://github.com/InjectiveLabs/swap-contract) | More complex atomic swaps over multiple market hops.                                        |

## Development

### Environment Setup

- Rust v1.69.0
- `wasm32-unknown-unknown` target
- Docker

1. Install [`rustup`](https://rustup.rs)
2. Run the following

```shell
rustup default 1.69.0
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
  cosmwasm/workspace-optimizer:0.13.0
```

This performs several optimizations which can significantly reduce the final size of the contract binaries, which will
be available inside the `artifacts/` directory.

## Formatting

Make sure you run `rustfmt` before creating a PR to the repo.

To run `rustfmt`,

```
cargo fmt
```

## Linting

You should run `clippy` also. This is a lint tool for Rust. It suggests more efficient/readable code. You can
see [the clippy document](https://rust-lang.github.io/rust-clippy/master/index.html) for more information.

### Run

```
cargo clippy -- -D warnings
```

## Testing

Developers are strongly encouraged to write unit tests for new code, and to submit new unit tests for old code. Unit
tests can be compiled and run with: `cargo test --all`. For more details, please reference Unit Tests.
