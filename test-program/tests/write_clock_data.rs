mod common;

use {
    common::process_fixture,
    protosol::fixture::{
        context::FixtureContext, effects::FixtureEffects, sysvars::FixtureSysvarContext, Fixture,
    },
    solana_sdk::{
        account::{Account, AccountSharedData},
        feature_set::FeatureSet,
        instruction::AccountMeta,
        pubkey::Pubkey,
    },
    std::vec,
};

#[test]
fn test_write_clock_data() {
    let pubkey = Pubkey::new_unique();
    let clock_slot = 123;

    // Set up the sysvar context
    let mut sysvar_context = FixtureSysvarContext::default();
    sysvar_context.clock.slot = clock_slot;

    let account_inputs = vec![(
        pubkey,
        AccountSharedData::from(Account {
            data: vec![0; 8],
            lamports: 100_000_000,
            owner: test_program::id(),
            ..Account::default()
        }),
    )];

    let instruction_accounts = vec![AccountMeta::new(pubkey, false)];

    let modified_accounts = vec![(
        pubkey,
        AccountSharedData::from(Account {
            data: clock_slot.to_le_bytes().to_vec(), // Clock slot should be written.
            lamports: 100_000_000,
            owner: test_program::id(),
            ..Account::default()
        }),
    )];

    let fixture = Fixture {
        input: FixtureContext {
            program_id: test_program::id(),
            loader_id: solana_sdk::bpf_loader_upgradeable::id(),
            feature_set: FeatureSet::all_enabled(),
            sysvar_context,
            accounts: account_inputs,
            instruction_accounts,
            instruction_data: vec![
                2, // WriteClockData
            ],
        },
        output: FixtureEffects {
            result: 0,
            custom_error: 0,
            modified_accounts,
        },
    };

    process_fixture(fixture);
}

#[test]
fn test_write_clock_data_fail_bad_owner() {
    let pubkey = Pubkey::new_unique();

    let account_inputs = vec![(
        pubkey,
        AccountSharedData::from(Account {
            data: vec![0; 4],
            lamports: 100_000_000,
            owner: Pubkey::new_unique(), // Incorrect owner
            ..Account::default()
        }),
    )];

    let instruction_accounts = vec![AccountMeta::new(pubkey, false)];

    let fixture = Fixture {
        input: FixtureContext {
            program_id: test_program::id(),
            loader_id: solana_sdk::bpf_loader_upgradeable::id(),
            feature_set: FeatureSet::all_enabled(),
            sysvar_context: FixtureSysvarContext::default(),
            accounts: account_inputs,
            instruction_accounts,
            instruction_data: vec![
                2, // WriteClockData
            ],
        },
        output: FixtureEffects {
            result: -1,                // -1 for failure
            custom_error: 30064771072, // `ProgramError::IncorrectProgramId`
            modified_accounts: vec![],
        },
    };

    process_fixture(fixture);
}

#[test]

fn test_write_clock_data_fail_bad_account_data() {
    let pubkey = Pubkey::new_unique();

    let account_inputs = vec![(
        pubkey,
        AccountSharedData::from(Account {
            data: vec![0; 2], // Incorrect data length
            lamports: 100_000_000,
            owner: test_program::id(),
            ..Account::default()
        }),
    )];

    let instruction_accounts = vec![AccountMeta::new(pubkey, false)];

    let fixture = Fixture {
        input: FixtureContext {
            program_id: test_program::id(),
            loader_id: solana_sdk::bpf_loader_upgradeable::id(),
            feature_set: FeatureSet::all_enabled(),
            sysvar_context: FixtureSysvarContext::default(),
            accounts: account_inputs,
            instruction_accounts,
            instruction_data: vec![
                2, // WriteClockData
            ],
        },
        output: FixtureEffects {
            result: -1,                // -1 for failure
            custom_error: 17179869184, // `ProgramError::InvalidAccountData`
            modified_accounts: vec![],
        },
    };

    process_fixture(fixture);
}
