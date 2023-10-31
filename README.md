# Learn Rocket in Rust

## Development Environment Setup

1. `cd` into the directory.
2. If you do not have the latest `nightly` channel release of RustC, install it.
3. `rustup override set nightly` to set the current repository to use the `nightly` build of Rust.
4. `rustup update && cargo update` to update to the latest `nightly` build.
    - __IMPORTANT:__ Rocket will not work without the latest `nightly` build.
5. `cargo build`
6. `cargo run`
