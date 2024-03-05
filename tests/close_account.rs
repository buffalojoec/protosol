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
fn test_write_data() {
    let pubkey = Pubkey::new_unique();
    let destination = Pubkey::new_unique();

    let account_inputs = vec![
        (
            pubkey,
            AccountSharedData::from(Account {
                data: vec![1, 2, 3, 4, 5, 6, 7, 8],
                lamports: 100_000_000,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
        (destination, AccountSharedData::default()),
    ];

    let instruction_accounts = vec![
        AccountMeta::new(pubkey, false),
        AccountMeta::new(destination, false),
    ];

    let modified_accounts = vec![
        (
            pubkey,
            AccountSharedData::default(), // Account should be closed.
        ),
        (
            destination,
            AccountSharedData::from(Account {
                lamports: 100_000_000, // Account should receive lamports.
                ..Account::default()
            }),
        ),
    ];

    let fixture = Fixture {
        input: FixtureContext {
            program_id: test_program::id(),
            loader_id: solana_sdk::bpf_loader_upgradeable::id(),
            feature_set: FeatureSet::all_enabled(),
            sysvar_context: FixtureSysvarContext::default(),
            accounts: account_inputs,
            instruction_accounts,
            instruction_data: vec![
                3, // CloseAccount
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
fn test_close_account_fail_bad_owner() {
    let pubkey = Pubkey::new_unique();
    let destination = Pubkey::new_unique();

    let account_inputs = vec![
        (
            pubkey,
            AccountSharedData::from(Account {
                data: vec![1, 2, 3, 4, 5, 6, 7, 8],
                lamports: 100_000_000,
                owner: Pubkey::new_unique(), // Incorrect owner
                ..Account::default()
            }),
        ),
        (destination, AccountSharedData::default()),
    ];

    let instruction_accounts = vec![
        AccountMeta::new(pubkey, false),
        AccountMeta::new(destination, false),
    ];

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
