# Frontend Compiler for DBSP

[Database Stream Processor (DBSP)](https://github.com/vmware/database-stream-processor) is a high-performance runtime for streaming computations.
DBSP provides a low-level Rust API to transform, filter, join, aggregate, etc., streams of data in real time.  This repository contains an infrastructure to run programs written in high-level languages on top of DBSP.  We aim to support several such languages:

- SQL (lives in a separate repository, will be open sourced soon)
- [Differential Datalog v2 (DDlog-2)](crates/ddlog-syntax) - an evolution of [DDlog](https://github.com/vmware/differential-datalog), a Datalog-inspired declarative language
- Functional languages like Scala (part of a future plan)

In particular, this repository will define:

- An intermediate representation (IR) of a DBSP program
- A frontend compiler from DDlog-2 syntax to the IR
- A language server infrastructure to enable DDlog-2 support in modern IDEs like Visual Studio Code
- A query optimizer and compiler backend to execute IR on top of the DBSP runtime in either compiled or interpreted mode.

## Developing

### Prerequisites

- `rustc` and `cargo` for building the language server and compiler, it's recommended to install them via [`rustup`](https://rustup.rs/)
- [`rustfmt`](https://github.com/rust-lang/rustfmt#quick-start) and [`clippy`](https://github.com/rust-lang/rust-clippy#as-a-cargo-subcommand-cargo-clippy) for formatting and linting the language server and compiler
- [`npm`](https://www.npmjs.com/) for building the language server extension

### Running, Building and Testing

We use `cargo` for the majority of our building and testing infrastructure

```bash
# Run all tests
$ cargo test

# Build the language server (add the `--release` flag for a release build)
$ cargo build --bin ddlog-driver

# Run the language server (add the `--release` flag for a release build)
$ cargo run --bin ddlog-driver
```

To enable logging while running tests or binaries, set the `DDLOG_LOG` flag to a
[logging directive](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives),
a good starting point would be `DDLOG_LOG=info`.
The five supported error levels are `error`, `warn`, `info`, `trace` and `debug`. Setting the log level will enable all logs
that occur at that level or higher, e.g. setting the log level to `info` will also enable all logs with the `error` or `warn`
levels.

### Tooling

Developer tooling is provided by the `xtask` crate

```bash
# See all available xtask commands
$ cargo xtask help

# Set up the repo for development
$ cargo xtask setup

# Run code generation
$ cargo xtask codegen

# Format all rust code
$ cargo fmt

# Lint all rust code
$ cargo clippy
```
