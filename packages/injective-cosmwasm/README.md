# injective-cosmwasm

<div align="center">
<h1 align="center">
<img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" />
<br></h1>
<h3>◦ injective-cosmwasm</h3>

<p align="center">
<img src="https://img.shields.io/badge/Rust-000000.svg?style=flat-square&logo=Rust&logoColor=white" alt="Rust" />
</p>
</div>

---

## 📖 Table of Contents

- [📖 Table of Contents](#-table-of-contents)
- [📍 Overview](#-overview)
- [📦 Features](#-features)
- [📂 repository Structure](#-repository-structure)
- [⚙️ Modules](#modules)
- [🚀 Getting Started](#-getting-started)
  - [🔧 Installation](#-installation)
  - [🤖 Running ](#-running-)
  - [🧪 Tests](#-tests)
- [🛣 Roadmap](#-roadmap)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)
- [👏 Acknowledgments](#-acknowledgments)

---

## 📍 Overview

`injective-cosmwasm` is designed for integration with the Injective chain, thereby enabling smart contract interactions and complex financial operations. It is a Rust-based library to be used with CosmWasm smart contract. It provides bindings for Injective-specific queries and messages, as well as some helpers.

---

## 📦 Features

Exception:

---

## 📂 Repository Structure

```sh
└── /
    ├── Cargo.toml
    └── src/
        ├── authz/
        │   ├── mod.rs
        │   └── response.rs
        ├── exchange/
        │   ├── cancel.rs
        │   ├── derivative.rs
        │   ├── derivative_market.rs
        │   ├── market.rs
        │   ├── mod.rs
        │   ├── order.rs
        │   ├── privileged_action.rs
        │   ├── response.rs
        │   ├── spot.rs
        │   ├── spot_market.rs
        │   ├── subaccount.rs
        │   └── types.rs
        ├── exchange_mock_querier.rs
        ├── lib.rs
        ├── msg.rs
        ├── oracle/
        │   ├── mod.rs
        │   ├── response.rs
        │   ├── types.rs
        │   └── volatility.rs
        ├── querier.rs
        ├── query.rs
        ├── route.rs
        ├── test_helpers.rs
        ├── tokenfactory/
        │   ├── mod.rs
        │   ├── response.rs
        │   └── types.rs
        ├── vesting/
        └── wasmx/
            ├── mod.rs
            ├── response.rs
            └── types.rs

```

---

## ⚙️ Modules

<details closed><summary>Root</summary>

| File                      | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Cargo.toml]({file_path}) | The Rust code configures a package named `injective-cosmwasm`, designed to provide bindings for CosmWasm contracts to interact with Injective Core's custom modules. Authored by contributors from InjectiveLabs, it is open-source with an Apache 2.0 license. The library uses the 2021 edition of Rust and includes various dependencies for blockchain and serialization functionality, such as `cosmwasm-std` and `serde`. It supports features like aborting, iterators, and Stargate, with compatibility for multiple CosmWasm versions. |

</details>

<details closed><summary>Src</summary>

| File                                    | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| --------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [route.rs]({file_path})                 | The code defines an `InjectiveRoute` enum that represents different query route paths for an Injective protocol, such as `Authz`, `Exchange`, `Tokenfactory`, `Staking`, `Oracle`, and `Wasmx`. It is serialized using a case-insensitive format suitable for URL path segments and can be used to match query types with their respective modules within the project directory. The provided directory tree suggests the project is a Rust-based trading/financial application with a focus on areas like authorization, market exchange, and oracles.                                                                                                                                                                                                                                                                                |
| [lib.rs]({file_path})                   | The code is a Rust module that defines the public interface and dependencies for a trading platform on a blockchain that supports injective protocols. It exports types, messages, and queries for working with derivative and spot markets, including order creation, cancellation, and position management. The module also handles oracle price feeds, market queries, and authz (authorization) functionalities. Utilities for testing are included but gated behind a non-WASM target condition. Core features like market data, account management, and trade execution are encapsulated within separate submodules. The platform interfaces with an external Injective blockchain protocol through a specified query and messaging system.                                                                                      |
| [query.rs]({file_path})                 | This Rust code defines a custom query system for the Injective Protocol, handling queries related to authorization (authz), exchange operations (including spot and derivative markets, orders, cancellations, positions, and pricing), staking, oracle data coordination, and specific web assembly module information (wasmx). It models a range of requests encapsulated in `InjectiveQuery`, which can query various parameters and states within the trading platform. These include user permissions, market specifics, order books, trade volume, pricing, oracle volatility, and contractual details, all accessed through a `InjectiveQueryWrapper` with distinct routing for each query type.                                                                                                                                |
| [test_helpers.rs]({file_path})          | The `test_helpers.rs` module in a Rust project provides testing utilities for a blockchain-based exchange platform. It includes constants for test market IDs and contract addresses, along with functions to generate a mock testing environment (`inj_mock_env`), manipulate dependencies (`OwnedDepsExt`), and create a `MockApi`, `MockStorage`, and custom querier (`inj_mock_deps`). Additionally, it provides a way to create mock spot market instances (`create_mock_spot_market`) with predefined parameters for testing market operations. The module is architected for non-WebAssembly targets and leverages the `cosmwasm_std` and `injective_math` crates for blockchain and mathematical operations, respectively.                                                                                                     |
| [msg.rs]({file_path})                   | The code provides a set of message constructors for a blockchain-related application, specifically for the Cosmos SDK with customizations for the Injective protocol. It defines an `InjectiveMsg` enum with various transaction types for managing subaccounts, token transfers, order creation/cancellation, market operations, and administrative actions for both spot and derivative markets. Additionally, it furnishes functions to create Cosmos messages wrapped in `InjectiveMsgWrapper` to be dispatched within the blockchain network. The code handles deposit, withdrawal, subaccount transfer, spot and derivative market orders, liquidation, reward opt-out, and contract activation, among other functionalities. It includes custom serialization and integration with the blockchain's query and execution layers. |
| [querier.rs]({file_path})               | The provided Rust code defines a `InjectiveQuerier` struct that offers various methods for querying blockchain data, interfacing with modules such as `Authz`, `Exchange`, `Oracle`, `Tokenfactory`, and `Wasmx`. These methods assemble requests, querying for things like grant permissions, exchange parameters, market information, deposited funds, order books, market volatilities, and oracle prices. The querier is designed for an environment where access to the blockchain state is facilitated through a wrapper that translates these queries into requests that can be understood by the underlying infrastructure. The code ensures type safety and modular interaction with different parts of the blockchain.                                                                                                       |
| [exchange_mock_querier.rs]({file_path}) | This Rust module provides a mock querier (`WasmMockQuerier`) to simulate blockchain queries during testing of a trading platform. The querier can handle various financial and blockchain-related queries including market data (spot and derivatives), order books, account balances, token supplies, and oracle prices. It uses mock API and storage to mimic the behavior of a blockchain, leveraging the `InjectiveQueryWrapper` type to represent custom query functionality specific to the Injective protocol.                                                                                                                                                                                                                                                                                                                  |

</details>

<details closed><summary>Wasmx</summary>

| File                       | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [response.rs]({file_path}) | The provided Rust project structure implements a trading exchange with authorization, market handling (spot and derivative), subaccounts, and an oracle for data. It includes a mock querier for testing and auxiliary modules for message routing, querying, and helpers. The code snippet defines a serializable response type for querying contract registration information within the `wasmx` module, including an optional `RegisteredContract`.                                                                                                                                 |
| [types.rs]({file_path})    | The code defines data structures in Rust for managing smart contracts on a blockchain platform. It includes an `enum` for funding modes (`FundingMode`) with options like `SelfFunded` and `GrantOnly`, and a `struct` (`RegisteredContract`) describing contract properties such as gas limits, prices, executability, and administrative controls. The `RegisteredContract` also includes optional fields for specifying code identifiers and administrative or granter addresses, with serialization supported by `serde` and `JsonSchema` for JSON compatibility.                  |
| [mod.rs]({file_path})      | This Rust project is structured into multiple modules that handle different aspects of a trading platform. The core functionalities likely include authorization (`authz`), market trading operations for both spot and derivatives (`exchange`), querying off-chain data (`oracle`), mocks for testing (`exchange_mock_querier`), message handling (`msg`), and abstractions for smart contract interactions (`wasmx`). Each module contains types and responses specific to its domain. The main library entry point is `lib.rs`, while `mod.rs` files serve as module declarations. |

</details>

<details closed><summary>Exchange</summary>

| File                                | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [response.rs]({file_path})          | The Rust code defines serialization for various response structures used in a crypto exchange platform. These responses include exchange parameters, subaccount deposits, positions, market info (spot and derivatives), order details, market prices, volatility statistics, staked amounts, order books, aggregate volumes, denom decimals, and fee multipliers. Each structure is serializable, using libraries `serde` and `schemars`, and contains optional or vector-typed properties depending on the expected data, to be returned in crypto exchange-related queries.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| [types.rs]({file_path})             | The code in `types.rs` defines several data structures for a cryptocurrency exchange system using CosmWasm, which is a smart contracting platform compatible with the Cosmos ecosystem. Key structures include:-`Params`: Configures exchange parameters such as fees, margin ratios, and access levels for different trading and listing operations.-`Deposit`: Represents a subaccount's deposit details with available and total balances.-`DenomDecimals`: Stores decimal precision for a currency denomination.-`PriceLevel`: Represents a price level with price (p) and quantity (q).-`VolumeByType`: Distinguishes between maker and taker trading volumes.-`MarketVolume`: Combines `VolumeByType` with a market identifier.-`MarketType`: Enum distinguishing Spot and Derivative markets.-`AtomicMarketOrderAccessLevel`: Enum for market order access control.-`MarketId`: Represents a market ID with checks for a 0x prefix and fixed length.-`SubaccountId`: Represents a subaccount ID with similar validation as `MarketId`.-`ShortSubaccountId`: Shortened subaccount ID for quick access, with validation and serialization methods.-`Hash`: Encapsulates a 32-byte hash, allowing for hexadecimal conversion.The code ensures data integrity through custom serialization/deserialization logic and provides helper functions and methods for data manipulation involving market and subaccount IDs. It also includes thorough unit tests to validate the correctness of these components. |
| [market.rs]({file_path})            | The `market.rs` file defines a `MarketStatus` enum representing various states of a market, and a `GenericMarket` trait outlining the essential functionality for a market entity in a financial exchange system. This includes retrieving market identifiers, status, ticker information, fee rates, and minimum quote increments. The `MarketStatus` has five states, including an `Unspecified` default. The properties and actions related to markets are given in terms of abstract operations, likely to be implemented by specific market types.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| [order.rs]({file_path})             | The code provides structures and functionality for managing and querying order data within a cryptocurrency exchange system built on the Cosmos SDK. It includes enums for `OrderSide` and `OrderType` with serialization options; structures like `OrderData`, `ShortOrderData`, `OrderInfo`, and `ShortOrderInfo` for holding order details with conversion implementations; traits `GenericOrder` and `GenericTrimmedOrder` defining common behaviors for various order types; and a test case ensuring correct serialization of `OrderType`. It utilizes external libraries like `cosmwasm_std`, `injective_math`, and `serde` for blockchain interaction, precise decimal arithmetic, and JSON serialization respectively.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| [derivative_market.rs]({file_path}) | The provided Rust code defines data structures and an implementation for managing derivative markets within a trading platform. It includes details for perpetual markets such as funding rates and intervals, and comprehensive market information like ticker, fees, margin ratios, and status. Derivative markets can also be perpetual, and their pricing is linked to an oracle system. The structures are serializable and include traits for accessing key market parameters such as fees, ticker, and status, ensuring integration with broader system functionalities.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| [cancel.rs]({file_path})            | The provided Rust module `cancel.rs` defines an enumeration `CancellationStrategy` with serialization capabilities, representing strategies for order cancellation within a trading exchange context. The strategies include canceling unspecified orders, from worst to best, and from best to worst price conditions. This code is part of a larger project structure focused on cryptocurrency exchange operations, including authorization, market types, order management, and auxiliary services.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| [subaccount.rs]({file_path})        | The code defines functions for managing subaccount IDs within a trading system, specifically converting between human-readable, Bech32 Cosmos addresses and Ethereum hex addresses. It generates a unique subaccount ID from a Cosmos address, using an optional nonce. It also checks whether a subaccount ID is a default one, based on its nonce, and converts subaccount IDs to either Ethereum or Bech32 Cosmos addresses. The included tests validate these conversion operations. Error handling assumes valid inputs and fails otherwise.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [spot_market.rs]({file_path})       | The provided Rust code defines a `SpotMarket` struct with properties for a financial spot market, including fees, denominations, and tick sizes, and implements the `GenericMarket` trait for common market operations. It also includes a function `calculate_spot_market_id` to generate a market ID based on the concatenation of base and quote denominations using a Keccak hash. A test verifies the ID generation correctness for a given base and quote. The entire code is part of an exchange module within a larger application, likely related to cryptocurrency trading.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| [mod.rs]({file_path})               | The provided code structure represents a Rust project with a focus on cryptocurrency exchange functionality. The `exchange` module, defined in `src/exchange/mod.rs`, is a central part of the application, organizing related exchange features such as order management, market types (spot and derivative markets), subaccounts, and the cancellation and privileged actions on orders, among other things. Each feature has a dedicated module within the `exchange` directory suggesting a modular codebase designed for handling various aspects of trading operations within a cryptocurrency exchange platform.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| [derivative.rs]({file_path})        | The Rust code defines data structures and methods for managing orders and positions in a derivatives exchange system. It includes types such as `Position`, `DerivativePosition`, `DerivativeOrder`, `EffectivePosition`, `ShortDerivativeOrder`, `DerivativeLimitOrder`, and `DerivativeMarketOrder`, along with traits `GenericOrder` and `GenericTrimmedOrder`. Positions calculate value with or without funding, apply funding, and are identified by market and subaccount. Orders include order types, price, quantity, margin, and optional trigger prices. Orders can be checked for validity, whether they are buy/sell, reduce-only, post-only, or atomic, and can be converted to shortened forms. Functionality for placing, managing, and valuing derivative trading contracts is encapsulated, featuring operations like creating new orders, calculating position or order values, and applying funding adjustments.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| [spot.rs]({file_path})              | The code defines data structures and behaviors for different kinds of spot market orders in a trading platform, using Rust with libraries for blockchain-related functionalities. `SpotLimitOrder` and `SpotMarketOrder` represent limit and market orders with properties like order type, price, quantity, fillability, and a trigger price. `SpotOrder` encapsulates basic order details without fillable quantity, while `ShortSpotOrder` presents a more concise version of `SpotOrder`. Conversions between order types are supported. Order trait implementations provide common behaviors to determine order characteristics (e.g., buy/sell, type, price). `MsgCreateSpotMarketOrderResponse` wraps the response for creating a market order, including the order hash and execution results.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| [privileged_action.rs]({file_path}) | The Rust module `privileged_action.rs` defines data structures for representing synthetic trades and position transfers in a financial exchange context, utilizing custom fixed-point decimal types for precision. `SyntheticTrade` holds trade details, `SyntheticTradeAction` aggregates user and contract trades, while `PositionTransferAction` describes the transfer of a position from one subaccount to another. `PrivilegedAction` optionally combines synthetic trades and position transfers. Additionally, there's a utility function `coins_to_string` to convert a list of `Coin` objects to a comma-separated string.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

</details>

<details closed><summary>Oracle</summary>

| File                         | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [response.rs]({file_path})   | The Rust code defines two data structures for oracle responses, `OraclePriceResponse` and `PythPriceResponse`, which contain optional fields representing the state of price pairs and Pyth price, respectively. Both structures are serializable, facilitating compatibility with JSON-based protocols and schema generation for API documentation. This is part of a larger financial trading platform, as indicated by the directory structure including modules for authorization, market exchange types, oracles, and token factory.                                                                  |
| [types.rs]({file_path})      | The provided code defines data structures for querying and handling oracle price data within a Rust-based blockchain or financial application. It includes types for representing oracle information, historical options, and responses, including volatility and pricing data. There is also a variety of oracle types, along with Pyth-specific types, which encapsulate individual price attestations, market status, and aggregated pricing data. The common theme is serialization and schema support for these structures, likely for communication over a network or interaction with a blockchain. |
| [volatility.rs]({file_path}) | The `volatility.rs` module defines structures to model and serialize metadata statistics, trade history options, price, and trade records pertaining to an oracle in a Rust project focused on exchange operations. These entities encompass data such as count, sample size, mean, timestamps, price statistics, trade grouping, raw history inclusion, and quantity of traded assets, leveraging `FPDecimal` for financial precision.                                                                                                                                                                    |
| [mod.rs]({file_path})        | The directory structure indicates a Rust project with multiple modules, focused on a trading exchange with features for spot and derivative markets, authorization, and an oracle for external data. The `src` directory contains the main library and modules, each with response handlers and type definitions. The `oracle/mod.rs` file serves as the module declaration for the oracle, pulling in response handling, types, and volatility-related functionality.                                                                                                                                     |

</details>

<details closed><summary>Tokenfactory</summary>

| File                       | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [response.rs]({file_path}) | The code defines two Rust data structures representing responses for a blockchain-based token factory module, using standard serialization libraries. `TokenFactoryDenomSupplyResponse` encapsulates the total supply of a token denomination, while `TokenFactoryCreateDenomFeeResponse` details the fee required to create a new token denomination, both as vectors of `Coin` structures. These structures are part of a larger financial trading platform, as denoted by sibling directories such as authz, exchange, and oracle.                                                                                                            |
| [types.rs]({file_path})    | The provided directory tree structure outlines a Rust project that includes a Cargo.toml for dependency management. The source folder (src) contains several modules, including authz, exchange, oracle, tokenfactory, vesting, and wasmx, each dedicated to different functionalities such as authorization, trading mechanisms, queries to oracles, token factory operations, vesting schedules, and interactions with WebAssembly modules, respectively. The specified file (src/tokenfactory/types.rs) likely contains type definitions for the tokenfactory module, which would be used for creating and managing tokens within the system. |
| [mod.rs]({file_path})      | The code represents a modular Rust project structure for a trading platform with authorization, exchange functionality including derivative and spot markets, and an oracle for data services. Components include market management, order processing, privileged actions, types definitions, mock queriers, messaging, queries, routing, and token factory management. The specific file `src/tokenfactory/mod.rs` declares the `response` module as part of the token factory subsystem.                                                                                                                                                       |

</details>

<details closed><summary>Authz</summary>

| File                       | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [response.rs]({file_path}) | The code defines Rust data structures for representing and serializing authorization grants and associated responses, including pagination. It includes the `Grant` structure detailing an authorization with its expiration, and `GrantAuthorization` which extends `Grant` with granter and grantee information. Additionally, `PageResponse` facilitates paginated results, while `GrantsResponse`, `GranteeGrantsResponse`, and `GranterGrantsResponse` wrap grant data with pagination details. The structures use Serde for serialization/deserialization and Schemars for JSON schema representations. |
| [mod.rs]({file_path})      | The code structure indicates a Rust project with a focus on a cryptocurrency exchange platform. Specifically, `src/authz/mod.rs` implies the authorization module may handle permissions and security, and it imports `response.rs` which might define response structs or enums for authorization operations. Other directories like `exchange`, `oracle`, and `tokenfactory` suggest functionalities for trading, market data, and token management, respectively, while `vesting` and `wasmx` imply features for token vesting and possibly WebAssembly integration.                                       |

</details>

---

## 🚀 Getting Started

**_Dependencies_**

Please ensure you have the following dependencies installed on your system:

`- ℹ️ Dependency 1`

`- ℹ️ Dependency 2`

`- ℹ️ ...`

### 🔧 Installation

1. Clone the repository:

```sh
git clone ../
```

2. Change to the project directory:

```sh
cd
```

3. Install the dependencies:

```sh
cargo build
```

### 🤖 Running

```sh
cargo run
```

### 🧪 Tests

```sh
cargo test
```

---

## 🤝 Contributing

Contributions are welcome! Here are several ways you can contribute:

- **[Submit Pull Requests](https://github.com/local//blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.
- **[Join the Discussions](https://github.com/local//discussions)**: Share your insights, provide feedback, or ask questions.
- **[Report Issues](https://github.com/local//issues)**: Submit bugs found or log feature requests for LOCAL.

#### _Contributing Guidelines_

<details closed>
<summary>Click to expand</summary>

1. **Fork the Repository**: Start by forking the project repository to your GitHub account.
2. **Clone Locally**: Clone the forked repository to your local machine using a Git client.
   ```sh
   git clone <your-forked-repo-url>
   ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
   ```sh
   git checkout -b new-feature-x
   ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear and concise message describing your updates.
   ```sh
   git commit -m 'Implemented new feature x.'
   ```
6. **Push to GitHub**: Push the changes to your forked repository.
   ```sh
   git push origin new-feature-x
   ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.

Once your PR is reviewed and approved, it will be merged into the main branch.

</details>

[**Return**](#Top)

---
