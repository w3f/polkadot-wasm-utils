# Polkadot Wasm Utilities by W3F TechEd Team

This is a WASM app using the [Yew UI framework](https://yew.rs/) using Rust based [Subxt](https://github.com/paritytech/subxt) library.
Deployed to [https://w3f.github.io/polkadot-utils](https://w3f.github.io/polkadot-utils/#/).

## Run locally

Update Rust:

```
rustup update
```

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

### On-demand Block Production on Paseo Network

Allows you to call `on_demand().place_order_keep_alive(max_amount, para_id)` and sign the transaction using an account from your browser extension.

