use {
    protosol::fixture::{
        context::FixtureContext, effects::FixtureEffects, sysvars::FixtureSysvarContext, Fixture,
    },
    solana_sdk::{
        account::{Account, AccountSharedData},
        feature_set::FeatureSet,
        instruction::Instruction,
        pubkey::Pubkey,
    },
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
                owner: solana_sdk::system_program::id(),
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: solana_sdk::system_program::id(),
                ..Account::default()
            }),
        ),
    ];

    let instruction =
        solana_sdk::system_instruction::transfer(&sender, &recipient, transfer_amount);
    let Instruction {
        accounts: instruction_accounts,
        data: instruction_data,
        ..
    } = instruction;

    let modified_accounts = vec![
        (
            sender,
            AccountSharedData::from(Account {
                lamports: base_lamports - transfer_amount, // Account should lose lamports.
                owner: solana_sdk::system_program::id(),
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports + transfer_amount, // Account should gain lamports.
                owner: solana_sdk::system_program::id(),
                ..Account::default()
            }),
        ),
    ];

    let output = FixtureEffects {
        result: 0,
        custom_error: 0,
        modified_accounts,
    };

    let fixture = Fixture {
        input: FixtureContext {
            program_id: solana_sdk::system_program::id(),
            loader_id: Pubkey::new_unique(), // Unused at the moment
            feature_set: FeatureSet::all_enabled(),
            sysvar_context: FixtureSysvarContext::default(),
            accounts: account_inputs,
            instruction_accounts,
            instruction_data,
        },
        output: output.clone(),
    };

    let resulting_effects = protosol::process_fixture(fixture);
    assert_eq!(resulting_effects, output);
}

#[test]
fn test_transfer_bad_owner() {
    let sender = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();

    let base_lamports = 100_000_000u64;
    let transfer_amount = 42_000u64;

    let account_inputs = vec![
        (
            sender,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: Pubkey::new_unique(), // Bad owner
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: solana_sdk::system_program::id(),
                ..Account::default()
            }),
        ),
    ];

    let instruction =
        solana_sdk::system_instruction::transfer(&sender, &recipient, transfer_amount);
    let Instruction {
        accounts: instruction_accounts,
        data: instruction_data,
        ..
    } = instruction;

    let output = FixtureEffects {
        result: -1,                         // -1 for failure
        custom_error: 18446744073709551615, // `InstructionError::ExternalAccountLamportSpend`
        modified_accounts: vec![],
    };

    let fixture = Fixture {
        input: FixtureContext {
            program_id: solana_sdk::system_program::id(),
            loader_id: Pubkey::new_unique(), // Unused at the moment
            feature_set: FeatureSet::all_enabled(),
            sysvar_context: FixtureSysvarContext::default(),
            accounts: account_inputs,
            instruction_accounts,
            instruction_data,
        },
        output: output.clone(),
    };

    let resulting_effects = protosol::process_fixture(fixture);
    assert_eq!(resulting_effects, output);
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
                owner: solana_sdk::system_program::id(),
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: solana_sdk::system_program::id(),
                ..Account::default()
            }),
        ),
    ];

    let mut instruction =
        solana_sdk::system_instruction::transfer(&sender, &recipient, transfer_amount);
    instruction.accounts[0].is_signer = false; // Sender is not a signer
    let Instruction {
        accounts: instruction_accounts,
        data: instruction_data,
        ..
    } = instruction;

    let output = FixtureEffects {
        result: -1,                // -1 for failure
        custom_error: 34359738368, // `ProgramError::MissingRequiredSignature`
        modified_accounts: vec![],
    };

    let fixture = Fixture {
        input: FixtureContext {
            program_id: solana_sdk::system_program::id(),
            loader_id: Pubkey::new_unique(), // Unused at the moment
            feature_set: FeatureSet::all_enabled(),
            sysvar_context: FixtureSysvarContext::default(),
            accounts: account_inputs,
            instruction_accounts,
            instruction_data,
        },
        output: output.clone(),
    };

    let resulting_effects = protosol::process_fixture(fixture);
    assert_eq!(resulting_effects, output);
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
                owner: solana_sdk::system_program::id(),
                ..Account::default()
            }),
        ),
        (
            recipient,
            AccountSharedData::from(Account {
                lamports: base_lamports,
                owner: solana_sdk::system_program::id(),
                ..Account::default()
            }),
        ),
    ];

    let instruction =
        solana_sdk::system_instruction::transfer(&sender, &recipient, transfer_amount);
    let Instruction {
        accounts: instruction_accounts,
        data: instruction_data,
        ..
    } = instruction;

    let output = FixtureEffects {
        result: -1,      // -1 for failure
        custom_error: 1, // `SystemError::ResultWithNegativeLamports`
        modified_accounts: vec![],
    };

    let fixture = Fixture {
        input: FixtureContext {
            program_id: solana_sdk::system_program::id(),
            loader_id: Pubkey::new_unique(), // Unused at the moment
            feature_set: FeatureSet::all_enabled(),
            sysvar_context: FixtureSysvarContext::default(),
            accounts: account_inputs,
            instruction_accounts,
            instruction_data,
        },
        output: output.clone(),
    };

    let resulting_effects = protosol::process_fixture(fixture);
    assert_eq!(resulting_effects, output);
}
