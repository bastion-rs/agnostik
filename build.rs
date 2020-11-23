use cfg_aliases::cfg_aliases;
use std::env;

const EXECUTOR_FEATURES: &[&str] = &[
    "CARGO_FEATURE_RUNTIME_BASTION",
    "CARGO_FEATURE_RUNTIME_TOKIO",
    "CARGO_FEATURE_RUNTIME_ASYNCSTD",
    "CARGO_FEATURE_RUNTIME_SMOL",
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if EXECUTOR_FEATURES.iter().filter_map(env::var_os).count() > 1 {
        panic!("you can only enable one runtime feature flag for agnostik");
    }

    cfg_aliases! {
        bastion: { feature = "runtime_bastion" },
        tokio: { feature = "runtime_tokio" },
        async_std: { feature = "runtime_asyncstd" },
        smol: { feature = "runtime_smol" },

        local_spawn: { any(tokio, async_std) },
        enable: { any(smol, tokio, async_std, bastion) },
    }
}
