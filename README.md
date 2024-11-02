# Polkadot Utilities by W3F TechEd Team

This is a WASM app using the [Yew UI framework](https://yew.rs/) using Rust based [Subxt](https://github.com/paritytech/subxt) library

To run the app locally we first install Trunk, a WASM bundler:

```
cargo install --locked trunk
```

Run the app locally with:

```
trunk serve --open
```
## List of Utilities

### Unlocking vested DOT on a specified account

Allows you to call `vesting().vest_other(destination)` and sign the transaction using an account from your browser extension.
