# Differential Datalog v2

The Language Server and Compiler architecture for Differential Datalog v2

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
