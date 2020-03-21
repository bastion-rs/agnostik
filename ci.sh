#!/usr/bin/env sh

cargo test --features=runtime_bastion
cargo test --features=runtime_asyncstd
cargo test --features=runtime_tokio
# Disabled until we have async / await in no_std
# cargo test --features=runtime_nostd


## Examples
cargo run --example bastion --features=runtime_bastion -- --nocapture
cargo run --example tokio --features=runtime_tokio -- --nocapture
cargo run --example asyncstd --features=runtime_asyncstd -- --nocapture
