## tendermint-abci

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Audit Status][audit-image]][audit-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust Stable][rustc-image]

**NB: Currently heavily under construction**

Crate for creating ABCI applications for use with Tendermint. See the
[Tendermint ABCI docs][abci-docs] for more details on ABCI.

## Requirements

- The latest stable version of Rust

## Features

This crate exposes an interface that allows for the construction of ABCI
applications that run using the Rust standard library (i.e. simple
multi-threaded applications), as well as using various `async` runtimes.

Currently, in terms of `async` runtimes, we support [Tokio] and [`async-std`].

These are all enabled/disabled by way of feature flags.

## Testing

To run integration tests for the Rust standard library (non-`async`,
multi-threaded), run:

```bash
cargo test --features client,runtime-std,echo-app
```

To run all integration tests for all supported `async` runtimes, run:

```bash
cargo test --all-features
```

To run Tokio-specific integration tests only, run:

```bash
cargo test --features async,client,runtime-tokio,echo-app
```

To run `async-std`-specific integration tests only, run:

```bash
cargo test --features async,client,runtime-async-std,echo-app
```

## License

Copyright © 2020 Informal Systems

Licensed under the Apache License, Version 2.0 (the "License");
you may not use the files in this repository except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/tendermint-abci.svg
[crate-link]: https://crates.io/crates/tendermint-abci
[docs-image]: https://docs.rs/tendermint-abci/badge.svg
[docs-link]: https://docs.rs/tendermint-abci/
[build-image]: https://github.com/informalsystems/tendermint-rs/workflows/Rust/badge.svg
[build-link]: https://github.com/informalsystems/tendermint-rs/actions?query=workflow%3ARust
[audit-image]: https://github.com/informalsystems/tendermint-rs/workflows/Audit-Check/badge.svg
[audit-link]: https://github.com/informalsystems/tendermint-rs/actions?query=workflow%3AAudit-Check
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/informalsystems/tendermint-rs/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-stable-blue.svg

[//]: # (general links)

[abci-docs]: https://docs.tendermint.com/master/spec/abci/
[Tokio]: https://tokio.rs/
[`async-std`]: https://async.rs/
