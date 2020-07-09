#!/usr/bin/env sh

cargo test --no-default-features --features=runtime_bastion
cargo test --no-default-features --features=runtime_asyncstd
cargo test --no-default-features --features=runtime_tokio
cargo test --no-default-features --features=runtime_smol
