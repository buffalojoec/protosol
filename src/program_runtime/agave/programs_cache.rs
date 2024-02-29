use {
    solana_bpf_loader_program::syscalls::create_program_runtime_environment_v1,
    solana_program_runtime::{
        compute_budget::ComputeBudget,
        invoke_context::BuiltinFunctionWithContext,
        loaded_programs::{LoadProgramMetrics, LoadedProgram, LoadedProgramsForTxBatch},
    },
    solana_sdk::{feature_set::FeatureSet, pubkey::Pubkey},
    std::sync::Arc,
};

// No need to import the Agave runtime for just the builtins.
struct Builtin {
    program_id: Pubkey,
    name: &'static str,
    entrypoint: BuiltinFunctionWithContext,
}

static BUILTINS: &[Builtin] = &[
    Builtin {
        program_id: solana_sdk::bpf_loader::id(),
        name: "solana_bpf_loader_program",
        entrypoint: solana_bpf_loader_program::Entrypoint::vm,
    },
    Builtin {
        program_id: solana_sdk::bpf_loader_upgradeable::id(),
        name: "solana_bpf_loader_upgradeable_program",
        entrypoint: solana_bpf_loader_program::Entrypoint::vm,
    },
    /* Additional builtins... */
];

pub fn build_loaded_programs_cache(
    program_id: &Pubkey,
    loader_id: &Pubkey,
    compute_budget: &ComputeBudget,
    feature_set: &FeatureSet,
    metrics: &mut LoadProgramMetrics,
    elf: &[u8],
) -> LoadedProgramsForTxBatch {
    let program_runtime_environment =
        create_program_runtime_environment_v1(feature_set, compute_budget, false, false).unwrap();

    let mut cache = LoadedProgramsForTxBatch::default();

    BUILTINS.iter().for_each(
        |Builtin {
             program_id,
             name,
             entrypoint,
         }| {
            cache.replenish(
                *program_id,
                Arc::new(LoadedProgram::new_builtin(0, name.len(), *entrypoint)),
            );
        },
    );

    cache.replenish(
        *program_id,
        Arc::new(
            LoadedProgram::new(
                loader_id,
                Arc::new(program_runtime_environment),
                0,
                0,
                None,
                elf,
                elf.len(),
                metrics,
            )
            .unwrap(),
        ),
    );

    cache
}
