use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        bastion: { feature = "runtime_bastion" },
        tokio: { feature = "runtime_tokio" },
        async_std: { feature = "runtime_asyncstd" },
        smol: { feature = "runtime_smol" },

        local_spawn: { any(smol, tokio) },
        global: { any(bastion, tokio, async_std, smol) },
    }
}
