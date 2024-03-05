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

static AGAVE_BUILTINS: &[Builtin] = &[
    Builtin {
        program_id: solana_system_program::id(),
        name: "system_program",
        entrypoint: solana_system_program::system_processor::Entrypoint::vm,
    },
    Builtin {
        program_id: solana_vote_program::id(),
        name: "vote_program",
        entrypoint: solana_vote_program::vote_processor::Entrypoint::vm,
    },
    Builtin {
        program_id: solana_stake_program::id(),
        name: "stake_program",
        entrypoint: solana_stake_program::stake_instruction::Entrypoint::vm,
    },
    Builtin {
        program_id: solana_config_program::id(),
        name: "config_program",
        entrypoint: solana_config_program::config_processor::Entrypoint::vm,
    },
    Builtin {
        program_id: solana_sdk::bpf_loader_deprecated::id(),
        name: "solana_bpf_loader_deprecated_program",
        entrypoint: solana_bpf_loader_program::Entrypoint::vm,
    },
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
    Builtin {
        program_id: solana_sdk::compute_budget::id(),
        name: "compute_budget_program",
        entrypoint: solana_compute_budget_program::Entrypoint::vm,
    },
    Builtin {
        program_id: solana_sdk::address_lookup_table::program::id(),
        name: "address_lookup_table_program",
        entrypoint: solana_address_lookup_table_program::processor::Entrypoint::vm,
    },
    Builtin {
        program_id: solana_zk_token_sdk::zk_token_proof_program::id(),
        name: "zk_token_proof_program",
        entrypoint: solana_zk_token_proof_program::Entrypoint::vm,
    },
    Builtin {
        program_id: solana_sdk::loader_v4::id(),
        name: "loader_v4",
        entrypoint: solana_loader_v4_program::Entrypoint::vm,
    },
];

/// Get the program account for a specified builtin.
pub fn builtin_program_account(
    program_id: &Pubkey,
    rent: &Rent,
) -> Vec<(Pubkey, AccountSharedData)> {
    let data = AGAVE_BUILTINS
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

    AGAVE_BUILTINS.iter().for_each(
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
