#!/usr/bin/env sh

cargo test --features=runtime_bastion
cargo test --features=runtime_asyncstd
cargo test --features=runtime_tokio
cargo test --features=runtime_nostd