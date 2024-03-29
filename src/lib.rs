//! Protobuf fuzzing & testing harness for Solana programs.

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod fixture;
mod program_accounts;
mod programs_cache;

use {
    crate::fixture::{context::FixtureContext, effects::FixtureEffects, Fixture},
    solana_program_runtime::{
        compute_budget::ComputeBudget,
        invoke_context::InvokeContext,
        loaded_programs::{LoadProgramMetrics, LoadedProgramsForTxBatch},
        sysvar_cache::SysvarCache,
        timings::ExecuteTimings,
    },
    solana_sdk::{
        hash::Hash,
        instruction::AccountMeta,
        program_error::ProgramError,
        transaction_context::{InstructionAccount, TransactionContext},
    },
    std::{collections::HashMap, sync::Arc},
};

/// Process a fixture using the simulated Solana program runtime.
pub fn process_fixture(fixture: Fixture, elf: &[u8]) {
    let Fixture { input, output } = fixture;
    let FixtureContext {
        program_id,
        loader_id,
        feature_set,
        sysvar_context,
        accounts,
        instruction_accounts: account_metas,
        instruction_data,
    } = input;
    let FixtureEffects {
        result: expected_result_code,
        custom_error: expected_error_code,
        modified_accounts: expected_modified_accounts,
    } = output;

    let compute_budget = ComputeBudget::default();
    let mut compute_units_consumed = 0;
    let mut load_program_metrics = LoadProgramMetrics::default();
    let mut programs_modified_by_tx = LoadedProgramsForTxBatch::default();
    let rent = sysvar_context.rent.clone();
    let sysvar_cache: SysvarCache = sysvar_context.into();
    let mut timings = ExecuteTimings::default();

    let program_accounts = program_accounts::program_accounts(&program_id, &loader_id, &rent, elf);
    let program_accounts_len = program_accounts.len();
    let program_indices = &[0];

    let instruction_accounts = account_metas
        .iter()
        .enumerate()
        .map(
            |(
                i,
                AccountMeta {
                    pubkey: _,
                    is_signer,
                    is_writable,
                },
            )| InstructionAccount {
                index_in_callee: i as u16,
                index_in_caller: i as u16,
                index_in_transaction: (i + program_accounts_len) as u16,
                is_signer: *is_signer,
                is_writable: *is_writable,
            },
        )
        .collect::<Vec<_>>();

    let transaction_accounts = program_accounts
        .into_iter()
        .chain(accounts)
        .collect::<Vec<_>>();

    let mut transaction_context = TransactionContext::new(
        transaction_accounts,
        rent,
        compute_budget.max_invoke_stack_height,
        compute_budget.max_instruction_trace_length,
    );

    let loaded_programs_cache = programs_cache::build_loaded_programs_cache(
        &program_id,
        &loader_id,
        &compute_budget,
        &feature_set,
        &mut load_program_metrics,
        elf,
    );

    let mut invoke_context = InvokeContext::new(
        &mut transaction_context,
        &sysvar_cache,
        None,
        compute_budget,
        &loaded_programs_cache,
        &mut programs_modified_by_tx,
        Arc::new(feature_set),
        Hash::default(),
        0,
    );

    let result = invoke_context.process_instruction(
        &instruction_data,
        &instruction_accounts,
        program_indices,
        &mut compute_units_consumed,
        &mut timings,
    );

    let (result_code, error_code): (i32, u64) = match result {
        Ok(()) => (0, 0),
        Err(err) => {
            if let Ok(program_err) = ProgramError::try_from(err) {
                (-1, u64::from(program_err))
            } else {
                (-1, u64::MAX)
            }
        }
    };

    let resulting_accounts = transaction_context
        .deconstruct_without_keys()
        .unwrap()
        .into_iter()
        .skip(program_accounts_len)
        .zip(account_metas.iter().map(|meta| meta.pubkey))
        .map(|(account, key)| (key, account))
        .collect::<HashMap<_, _>>();

    assert_eq!(result_code, expected_result_code);
    assert_eq!(error_code, expected_error_code);
    for (key, expected_modified_account) in expected_modified_accounts {
        if let Some(account) = resulting_accounts.get(&key) {
            assert_eq!(account, &expected_modified_account);
        }
    }
}
