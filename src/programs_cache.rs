//! Solana program runtime loaded programs cache.

use {
    solana_program_runtime::{
        invoke_context::BuiltinFunctionWithContext,
        loaded_programs::{LoadedProgram, LoadedProgramsForTxBatch},
    },
    solana_sdk::{
        account::{Account, AccountSharedData},
        pubkey::Pubkey,
        rent::Rent,
    },
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

/// Get the program account for a specified builtin.
pub fn program_account(program_id: &Pubkey, rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    let data = BUILTINS
        .iter()
        .find(|Builtin { program_id: id, .. }| id == program_id)
        .map(|Builtin { name, .. }| *name)
        .unwrap_or("unknown_builtin")
        .as_bytes()
        .to_vec();
    vec![(
        *program_id,
        AccountSharedData::from(Account {
            lamports: rent.minimum_balance(data.len()).max(1),
            data,
            owner: solana_sdk::native_loader::id(),
            executable: true,
            rent_epoch: 0,
        }),
    )]
}

/// Build the loaded programs cache with a provided program and the above
/// builtins.
pub fn build_loaded_programs_cache() -> LoadedProgramsForTxBatch {
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

    cache
}
