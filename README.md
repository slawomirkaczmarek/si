# si
Shipping Instructions (SI), a helper tool for Cargo.

Currently available subcommands:
- [`si-update`](#si-update)

## Subcommands
### `si-update`
Run `cargo-update` and then update dependencies in `Cargo.toml` that are behind the versions recorded in `Cargo.lock`.

This command is similar to [`cargo-edit`'s `cargo-upgrade`](https://github.com/killercup/cargo-edit#cargo-upgrade), but instead of directly editing the manifest file, it relies on `cargo-rm` and `cargo-add` to do so.
