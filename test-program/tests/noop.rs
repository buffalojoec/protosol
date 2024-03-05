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
};

#[test]
fn test_noop() {
    let pubkey1 = Pubkey::new_unique();
    let pubkey2 = Pubkey::new_unique();

    let account_inputs = vec![
        (
            pubkey1,
            AccountSharedData::from(Account {
                lamports: 100_000_000,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
        (
            pubkey2,
            AccountSharedData::from(Account {
                lamports: 100_000_000,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
    ];

    let instruction_accounts = vec![
        AccountMeta::new_readonly(pubkey1, false),
        AccountMeta::new_readonly(pubkey2, false),
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
                0, // NoOp
            ],
        },
        output: FixtureEffects {
            result: 0,
            custom_error: 0,
            modified_accounts: vec![], // NoOp should not modify accounts.
        },
    };

    process_fixture(fixture);
}

#[test]
fn test_noop_fail_bad_owners() {
    let pubkey1 = Pubkey::new_unique();
    let pubkey2 = Pubkey::new_unique();

    let create_failure_fixture = |account_inputs| {
        let instruction_accounts = vec![
            AccountMeta::new_readonly(pubkey1, false),
            AccountMeta::new_readonly(pubkey2, false),
        ];

        Fixture {
            input: FixtureContext {
                program_id: test_program::id(),
                loader_id: solana_sdk::bpf_loader_upgradeable::id(),
                feature_set: FeatureSet::all_enabled(),
                sysvar_context: FixtureSysvarContext::default(),
                accounts: account_inputs,
                instruction_accounts,
                instruction_data: vec![
                    0, // NoOp
                ],
            },
            output: FixtureEffects {
                result: -1,                // -1 for failure
                custom_error: 30064771072, // `ProgramError::IncorrectProgramId`
                modified_accounts: vec![],
            },
        }
    };

    // Fail if the first account's owner is not the program.
    process_fixture(create_failure_fixture(vec![
        (
            pubkey1,
            AccountSharedData::from(Account {
                lamports: 100_000_000,
                owner: Pubkey::new_unique(), // Incorrect owner
                ..Account::default()
            }),
        ),
        (
            pubkey2,
            AccountSharedData::from(Account {
                lamports: 100_000_000,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
    ]));

    // Fail if the second account's owner is not the program.
    process_fixture(create_failure_fixture(vec![
        (
            pubkey1,
            AccountSharedData::from(Account {
                lamports: 100_000_000,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
        (
            pubkey2,
            AccountSharedData::from(Account {
                lamports: 100_000_000,
                owner: Pubkey::new_unique(), // Incorrect owner
                ..Account::default()
            }),
        ),
    ]));
}
