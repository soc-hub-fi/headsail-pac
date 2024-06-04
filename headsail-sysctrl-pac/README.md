# SysCtrl PAC

Peripheral access API for System Control CPU of Headsail SoC. This file provides a guide for 1. users, 2. CI, 3. SVD update.

## Usage (users & CI)

We have two kinds of users. Identify yourself:

1. End-users: depend on the library.
2. CI: just build the library, perhaps run tests.

Both kinds of users need to have a Rust toolchain: run script at: <http://rustup.rs>.

### End-user guide

- Depend on this crate via Cargo.toml (e.g. by using the `git = ...` -specifier).
- See [SVD2Rust Peripheral API](https://docs.rs/svd2rust/*/svd2rust/index.html#peripheral-api)
for instructions on how to use the peripheral APIs.
- The primary documentation for this crate can be generated and opened with `cargo doc --open`.

### CI guide

- Build: `cargo build`
- Test: `cargo test`
- Generate HTML documentation at "target/doc": `cargo doc`

## Generate (SVD update guide)

### Pre-requirements

- `cargo install svd2rust --version="^0.33"`
- `cargo install form --version="^0.12"`
- `cargo install svdtools --version="^0.3"`

### Run generate

- Use `./update.sh` to re-generate Rust peripheral API from `headsail-sysctrl.svd`.
- `cargo check` to verify that the generated API compiles.

The `headsail-sysctrl.svd` represents the latest System View Description (SVD) of Headsail SoC for
the System Control CPU. This SVD is used to generate a Rust peripheral API using svd2rust. That code
is formatted and split into files using `form`. Finally, code is formatted to default Rust
convention using `cargo fmt`.

## Notes

- `NVIC_PRIO_BITS` is superfluous. It's required by the CMSIS-SVD format, but unnecessary on our platform.
