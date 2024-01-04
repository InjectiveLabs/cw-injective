# injective-math

<div align="center">
<h1 align="center">
<img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" />
<br></h1>
<h3>◦ injective-math</h3>

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

The repository hosts an advanced mathematical library primarily designed for financial calculations and scientific computing, with specific compatibility for CosmWasm contracts within the Injective Protocol ecosystem. It features a custom fixed-point decimal type (`FPDecimal`) supporting arithmetic, comparison, display formatting, serialization, root-finding algorithms, and vector operations. High precision and robust error handling are key attributes, which cater to the stringent accuracy requirements in financial transactions and smart contract development. Moreover, the inclusion of root-finding techniques and utilities for large-number arithmetic underscore its comprehensive nature in tackling a broad spectrum of mathematical challenges.

---

## 📦 Features

|     | Feature             | Description                                                                                                                                                           |
| --- | ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ⚙️  | **Architecture**    | A Rust-based library focused on high-precision arithmetic for fixed-point decimals, suitable for blockchain-related mathematical computations.                        |
| 📄  | **Documentation**   | Documentation within the code summarizes functionalities of the financial calculation library and its mathematical utilities. Incomplete at a high level.             |
| 🔗  | **Dependencies**    | External dependencies include `cosmwasm-std`, `ethereum-types`, and `serde` among others, configured for WebAssembly and blockchain integration.                      |
| 🧩  | **Modularity**      | Highly modular with separate files for arithmetic, comparison, scaling, rooting, vector computations, serialization, error handling, and more.                        |
| 🧪  | **Testing**         | Library includes unit tests for various mathematical functions, although test coverage is not assessed in the provided details.                                       |
| ⚡️ | **Performance**     | No explicit performance metrics provided, but the use of Rust and fixed-point arithmetic suggests a focus on computational efficiency.                                |
| 🔐  | **Security**        | Specific security measures aren't mentioned, but the controlled arithmetic mitigates certain numerical risks. Serialized data handling via `serde` requires scrutiny. |
| 🔀  | **Version Control** | Versioning follows semantic versioning with the current version at 0.2.3; version control strategies not specified in the details.                                    |
| 🔌  | **Integrations**    | Designed for integration with WebAssembly and the Injective Protocol, implying good interoperability within blockchain ecosystems.                                    |
| 📶  | **Scalability**     | The code structure and Rust usage imply scalability; however, real-world scalability is not explicit within the summaries.                                            |

The analysis above was generated based on the provided directory structure and code summaries for a Rust project focused on providing a library for fixed-point decimal arithmetic, primarily targeting blockchain applications. This interpretation of the provided data reflects a strong modular design, accompanied by a well-thought-out file structure to organize a wide range of mathematical computing features. It seems to target performance through Rust's efficiency and fine-grained error and type handling. Furthermore, the presence of serialization and deserialization capabilities via `serde` supports flexible data compatibility. Nevertheless, a deeper analysis, especially including a review of actual performance metrics, test coverage, security audits, and scalability testing, would be needed to provide a more precise evaluation.

---

## 📂 Repository Structure

```sh
└── /
    ├── .cargo/
    │   └── config
    ├── Cargo.toml
    └── src/
        ├── fp_decimal/
        │   ├── arithmetic.rs
        │   ├── comparison.rs
        │   ├── display.rs
        │   ├── error.rs
        │   ├── exp.rs
        │   ├── factorial.rs
        │   ├── from_str.rs
        │   ├── hyper.rs
        │   ├── log.rs
        │   ├── mod.rs
        │   ├── round.rs
        │   ├── scale.rs
        │   ├── serde.rs
        │   ├── trigonometry.rs
        │   └── utils.rs
        ├── lib.rs
        ├── root_findings.rs
        ├── utils.rs
        └── vector.rs

```

---

## ⚙️ Modules

<details closed><summary>Root</summary>

| File                      | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Cargo.toml]({file_path}) | The code represents a Rust project manifest for injective-math, a mathematical library tailored for CosmWasm contracts on the Injective Protocol. It specifies the package's metadata, including the author, license, description, and repository link. The project is configured for the Rust 2021 edition and has version 0.2.3. Features enable additional testing capabilities, and dependencies include specific versions of cosmwasm-std, ethereum-types, and others, with some having optional features. The project structure indicates modules for fixed-point decimals, utilities, root-finding algorithms, and vector operations. |

</details>

<details closed><summary>.cargo</summary>

| File                  | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [config]({file_path}) | This Rust project includes custom configurations and an extensive floating-point decimal library. The `.cargo/config` file defines build and test aliases for targeting WebAssembly and enabling features like backtraces. The `src` directory structure suggests the library provides a wide range of numerical operations, error handling, serialization support, and utilities, along with additional functionalities for root-finding and vector manipulation, indicating a focus on mathematical and scientific computing. |

</details>

<details closed><summary>Src</summary>

| File                            | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [vector.rs]({file_path})        | The provided Rust code defines a set of arithmetic operations for vectors of `FPDecimal` (a fixed-point decimal type). These operations include summing elements, dot product, element-wise multiplication and division by a constant, element-wise addition and subtraction, and computing the absolute values of elements within a vector. These functions facilitate vector-based calculations with fixed-point precision.                                                                                                                                                                                              |
| [lib.rs]({file_path})           | The code provides utilities for financial calculations with fixed-point decimals. It includes functions to calculate asset cluster imbalance using portfolio weights, prices, and inventories (`imbalance`), and to convert arrays of different integer types (`u32`, `Uint128`) and strings to arrays of `FPDecimal` objects (`int32_vec_to_fpdec`, `int_vec_to_fpdec`, `str_vec_to_fpdec`). The `imbalance` function performs elemental multiplications, a dot product, and scales for optimal capital allocation comparison.                                                                                            |
| [root_findings.rs]({file_path}) | The code implements root-finding algorithms—Newton's, discrete Newton's, and Halley's methods—for functions returning `FPDecimal` (a fixed-point decimal type). It supports iterating until a specified precision (`abs_error`) or number of iterations (`max_iter`) is met. The `newton` function calculates roots using a function and its derivative. The `discrete_newton` method approximates derivatives and solves for discrete functions. `halleys` extends Newton's method with the second derivative for faster convergence. It includes no support for complex numbers and test cases validating functionality. |
| [utils.rs]({file_path})         | The provided Rust code is part of a financial calculation library, consisting of utility functions for operating on fixed-precision decimal (FPDecimal) numbers. It includes functionalities like parsing decimals and integers within specified ranges, rounding to ticks with specific precision, and ensuring values fall within given bands. It handles errors and validation for inputs and avoids division by zero. The code is organized with tests verifying behaviors like flooring, rounding, division, and tick adjustments for precise financial operations.                                                   |

</details>

<details closed><summary>Fp_decimal</summary>

| File                           | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [display.rs]({file_path})      | The code defines a `Display` trait implementation for `FPDecimal`, a fixed-point decimal type, formatting it as a string with optional negative sign, integer part, and fractional part, omitting any trailing zeros in the fraction. It includes tests validating the correct string representation of positive, negative, and fractional `FPDecimal` values.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [serde.rs]({file_path})        | The `serde.rs` module provides serialization and deserialization implementations for `FPDecimal`, a custom fixed-precision decimal type, using Serde, a Rust serialization framework. Serialization converts `FPDecimal` instances into decimal strings, while deserialization constructs `FPDecimal` from string-encoded decimal values, with robust error handling for parsing failures. The module includes a custom visitor `FPDecimalVisitor` for deserialization tailored to the `FPDecimal` type, ensuring compatibility with Serde's data model.                                                                                                                                                                                                                                                                                                                    |
| [error.rs]({file_path})        | The provided code defines a Rust module structure for a numerical library, with a core directory focused on fixed-point decimal operations (`fp_decimal`). It includes functionality for arithmetic, comparison, display formatting, error handling, exponentiation, factorial calculation, string conversion, hyperbolic functions, logarithms, rounding, scaling, serialization, and trigonometry, among other utilities. The `error.rs` file within `fp_decimal` defines an error enumeration (`FPDecimalError`) for handling undefined and unsupported operations within the library.                                                                                                                                                                                                                                                                                   |
| [hyper.rs]({file_path})        | The code defines hyperbolic trigonometric functions (`sinh`, `cosh`, `tanh`) for a fixed-point decimal data type `FPDecimal` in a Rust module. It includes private functions with detailed implementations (`_sinh`, `_cosh`, `_tanh`) and corresponding public methods that call these functions, passing the instance as an argument. The implementation uses exponential functions, addition, subtraction, and division operations specific to `FPDecimal`. Unit tests confirm that these functions produce expected results for the value of 1.                                                                                                                                                                                                                                                                                                                         |
| [log.rs]({file_path})          | The `log.rs` module provides a natural logarithm function specifically for `FPDecimal` values, handling base `e` (Euler's number) and its powers up to \(e^{11}\), as well as their reciprocals, mapping them to their corresponding integral `FPDecimal` representations (0 through 11 and-1, respectively). Other values do not return a result, indicating a limitation in the logarithmic function's implementation for arbitrary `FPDecimal` instances.                                                                                                                                                                                                                                                                                                                                                                                                                |
| [arithmetic.rs]({file_path})   | The provided Rust code defines arithmetic operations for a fixed-point decimal represented by the `FPDecimal` struct within a numerical library, supporting addition, subtraction, multiplication, division, and modulus calculations. It handles numbers with potential sign differences and precision management. The implementation employs Rust's traits such as `Add`, `Sub`, `Mul`, `Div`, and their assign variants to integrate with native operators (`+`, `-`, `*`, `/`, etc.). Additional utility functions allow for absolute value determination, calculating absolute differences, and aggregation via `Sum`. Complex multiplication and division preserve precision, and various tests ensure the functionality's reliability and correctness.                                                                                                               |
| [from_str.rs]({file_path})     | The provided code is part of a Rust module that defines the `FromStr` trait implementation for a `FPDecimal` type, allowing the creation of this custom fixed-point decimal type from a string representation. It parses decimal strings (like `1.23`) without performing rounding, errors on invalid input, and supports up to 18 fractional digits. The `FPDecimal` struct has a `must_from_str` method that panics if the conversion fails. Unit tests validate parsing for negative, zero, and other decimal strings, ensuring accurate conversion to the internal representation.                                                                                                                                                                                                                                                                                      |
| [scale.rs]({file_path})        | The `scale.rs` code defines a trait `Scaled` for the `FPDecimal` type, allowing decimal scaling operations by multiplying by a power of ten. It includes an implementation to adjust the scale of `FPDecimal` instances and a function `dec_scale_factor` returning a set scaling factor of 10^18. Two tests verify functionality: one checks correct scaling up and down, and the other confirms the predefined factor value.                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [exp.rs]({file_path})          | The provided Rust code defines an `exp_taylor_expansion` method for the `FPDecimal` type, which computes an approximate value for the exponentiation a^b using the first 25 terms of the Taylor series expansion. The method calculates the natural logarithm of `a`, scales it by `b`, and iteratively constructs terms of the series by multiplying the numerator by the base and the denominator by consecutive integers from 2 to 25. It returns a vector of `FPDecimal` objects representing each term of the expansion.                                                                                                                                                                                                                                                                                                                                               |
| [mod.rs]({file_path})          | This Rust code defines a `FPDecimal` struct for fixed-point arithmetic, with conversions from standard integer and floating-point data types, as well as custom conversions for blockchain-related types like `Decimal256`, `Uint128`, and `Uint256`. It supports unary negation, provides constants for mathematical values, has utility functions for checking its state (zero, integer, negative), and handles overflow conditions. Unit tests affirm the correctness of constants, conversions, sign handling, integer checks, and rounding methods. The tree structure shows it being part of a larger library focused on expanded arithmetic and scientific computations.                                                                                                                                                                                             |
| [trigonometry.rs]({file_path}) | The provided Rust code defines a `FPDecimal` struct with methods to calculate sine and cosine functions imprecisely using Taylor series expansion, and an angle normalization method (`_change_range`) that wraps input angles into the range [0, 2π]. The sine (`_sin`) and cosine (`_cos`) methods call `_change_range` to normalize angles and handle specific angle cases directly for efficiency. Tests verify the accuracy within a 1% margin using an `almost_eq` function, demonstrating tests for 0, 1, and-1 angle inputs for both sine and cosine functions.                                                                                                                                                                                                                                                                                                     |
| [factorial.rs]({file_path})    | The code provides a `FPDecimal` struct with methods for calculating factorials and the gamma function. The factorial methods support positive and negative integers, using recursion, returning `FPDecimal::ONE` for zero input. The gamma function evaluates for non-integer values using a pre-calculated constants table, employing Horner's method for polynomial evaluation. Unit tests validate factorial computations for 9 and-9, ensuring correct parsing and method functionality within the `FPDecimal` context.                                                                                                                                                                                                                                                                                                                                                 |
| [comparison.rs]({file_path})   | The provided code defines Rust traits to compare `FPDecimal` instances, which seemingly represent fixed-point decimal numbers with `num` and `sign` attributes. The `Ord`, `PartialOrd`, and `PartialEq` traits are implemented to enable comparisons via standard operators (`<`, `<=`, `>`, `>=`, `==`, and `!=`). Additional methods `maximum` and `minimum` return the greater or lesser of two `FPDecimal` instances, respectively. Unit tests validate the correctness of these comparisons and methods.                                                                                                                                                                                                                                                                                                                                                              |
| [utils.rs]({file_path})        | The code snippet is from a Rust project structured to handle high-precision arithmetic operations, with a particular focus on a module named `fp_decimal`. The project is configured for Cargo (Rust's package manager), indicated by `.cargo/config` and `Cargo.toml` files. The `fp_decimal` directory under `src/` contains several Rust files (`*.rs`), each likely handling a different aspect of fixed-point decimal operations such as arithmetic, comparison, formatting, error handling, exponentiation, factorial calculation, parsing, logarithmic functions, and more. The presence of `serde.rs` suggests serialization/deserialization capabilities. The `FPDecimal` type and `U256` (a 256-bit unsigned integer type, probably from an external `bigint` library) are imported in `utils.rs`, indicating utility functions leveraging large-integer support. |
| [round.rs]({file_path})        | The provided directory tree and file path indicate a Rust project structure with a focus on fixed-precision decimal arithmetic. The code in `src/fp_decimal/round.rs` suggests implementation of rounding functions within a module `fp_decimal` of a numerical library, possibly providing precise arithmetic operations (e.g., addition, multiplication), special functions (e.g., exponentiation, logarithms), and utilities for handling fixed-precision decimal numbers, with `FPDecimal` as the main type and `U256` hinting at a 256-bit underlying numeric representation. The module may also support serialization with `serde.rs`.                                                                                                                                                                                                                               |

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

---

[**Return**](#Top)

---
