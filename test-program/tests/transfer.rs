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
fn test_transfer() {
    let sender = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();

    let base_lamports = 100_000_000u64;
    let transfer_amount = 42_000u64;

    let account_inputs = vec![
        (
            sender,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
    ];

    let instruction_accounts = vec![
        AccountMeta::new(sender, true),
        AccountMeta::new(recipient, false),
    ];

    let mut instruction_data = vec![4]; // Transfer
    instruction_data.extend_from_slice(&transfer_amount.to_le_bytes());

    let modified_accounts = vec![
        (
            sender,
            AccountSharedData::from(Account {
                lamports: base_lamports - transfer_amount, // Account should lose lamports.
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports + transfer_amount, // Account should gain lamports.
                owner: test_program::id(),
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
            instruction_data,
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
fn test_transfer_bad_owners() {
    let sender = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();

    let base_lamports = 100_000_000u64;
    let transfer_amount = 42_000u64;

    let create_failure_fixture = |account_inputs| {
        let instruction_accounts = vec![
            AccountMeta::new(sender, true),
            AccountMeta::new(recipient, false),
        ];

        let mut instruction_data = vec![4]; // Transfer
        instruction_data.extend_from_slice(&transfer_amount.to_le_bytes());

        Fixture {
            input: FixtureContext {
                program_id: test_program::id(),
                loader_id: solana_sdk::bpf_loader_upgradeable::id(),
                feature_set: FeatureSet::all_enabled(),
                sysvar_context: FixtureSysvarContext::default(),
                accounts: account_inputs,
                instruction_accounts,
                instruction_data,
            },
            output: FixtureEffects {
                result: -1,                // -1 for failure
                custom_error: 30064771072, // `ProgramError::IncorrectProgramId`
                modified_accounts: vec![],
            },
        }
    };

    // Fail if the sender's owner is not the program.
    process_fixture(create_failure_fixture(vec![
        (
            sender,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: Pubkey::new_unique(), // Incorrect owner
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
    ]));

    // Fail if the recipient's owner is not the program.
    process_fixture(create_failure_fixture(vec![
        (
            sender,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: Pubkey::new_unique(), // Incorrect owner
                ..Account::default()
            }),
        ),
    ]));
}

#[test]
fn test_transfer_sender_not_signer() {
    let sender = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();

    let base_lamports = 100_000_000u64;
    let transfer_amount = 42_000u64;

    let account_inputs = vec![
        (
            sender,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
    ];

    let instruction_accounts = vec![
        AccountMeta::new(sender, false), // Not a signer
        AccountMeta::new(recipient, false),
    ];

    let mut instruction_data = vec![4]; // Transfer
    instruction_data.extend_from_slice(&transfer_amount.to_le_bytes());

    let fixture = Fixture {
        input: FixtureContext {
            program_id: test_program::id(),
            loader_id: solana_sdk::bpf_loader_upgradeable::id(),
            feature_set: FeatureSet::all_enabled(),
            sysvar_context: FixtureSysvarContext::default(),
            accounts: account_inputs,
            instruction_accounts,
            instruction_data,
        },
        output: FixtureEffects {
            result: -1,                // -1 for failure
            custom_error: 34359738368, // `ProgramError::MissingRequiredSignature`
            modified_accounts: vec![],
        },
    };

    process_fixture(fixture);
}

#[test]
fn test_transfer_sender_not_enough_lamports() {
    let sender = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();

    let base_lamports = 100_000_000u64;
    let transfer_amount = 142_000_000u64; // Too much to transfer

    let account_inputs = vec![
        (
            sender,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
    ];

    let instruction_accounts = vec![
        AccountMeta::new(sender, true),
        AccountMeta::new(recipient, false),
    ];

    let mut instruction_data = vec![4]; // Transfer
    instruction_data.extend_from_slice(&transfer_amount.to_le_bytes());

    let fixture = Fixture {
        input: FixtureContext {
            program_id: test_program::id(),
            loader_id: solana_sdk::bpf_loader_upgradeable::id(),
            feature_set: FeatureSet::all_enabled(),
            sysvar_context: FixtureSysvarContext::default(),
            accounts: account_inputs,
            instruction_accounts,
            instruction_data,
        },
        output: FixtureEffects {
            result: -1,                // -1 for failure
            custom_error: 25769803776, // `ProgramError::34359738368`
            modified_accounts: vec![],
        },
    };

    process_fixture(fixture);
}
