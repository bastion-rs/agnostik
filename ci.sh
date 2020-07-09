#!/usr/bin/env sh

set -em

if [ "$1" == "check" ]; then
    cargo check --no-default-features --features=runtime_bastion
    cargo check --no-default-features --features=runtime_asyncstd
    cargo check --no-default-features --features=runtime_tokio
    cargo check --no-default-features --features=runtime_smol
elif [ "$1" == "test" ]; then
    cargo test --no-default-features --features=runtime_bastion
    cargo test --no-default-features --features=runtime_asyncstd
    cargo test --no-default-features --features=runtime_tokio
    cargo test --no-default-features --features=runtime_smol
else
    echo "You have to provide either 'check' or 'test' argument"
fi
