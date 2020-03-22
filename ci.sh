#!/usr/bin/env sh

cargo test --features=runtime_bastion
cargo test --features=runtime_asyncstd
cargo test --features=runtime_tokio
# Disabled until we have async / await in no_std
# cargo test --features=runtime_nostd
