#!/usr/bin/env sh

set -em

if [ "$1" = "check" ]; then
    cargo check
    cargo check --features=runtime_bastion
    cargo check --features=runtime_asyncstd
    cargo check --features=runtime_tokio
    cargo check --features=runtime_smol
elif [ "$1" = "test" ]; then
    cargo test
    cargo test --features=runtime_bastion
    cargo test --features=runtime_asyncstd
    cargo test --features=runtime_tokio
    cargo test --features=runtime_smol
else
    echo "You have to provide either 'check' or 'test' argument"
fi
