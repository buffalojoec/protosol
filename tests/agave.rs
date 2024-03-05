use {
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

fn get_test_elf<'a>() -> &'a [u8] {
    include_bytes!("test_program.so")
}

#[test]
fn test_process_instruction_fixture() {
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
        AccountMeta::new(pubkey1, true),
        AccountMeta::new(pubkey2, false),
    ];

    let account_outputs = vec![
        (
            pubkey1,
            AccountSharedData::from(Account {
                lamports: 0, // Account should have been emptied.
                owner: test_program::id(),
                ..Account::default()
            }),
        ),
        (
            pubkey2,
            AccountSharedData::from(Account {
                lamports: 200_000_000, // Account should have been doubled.
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
            instruction_data: vec![1, 2, 3, 4],
        },
        output: FixtureEffects {
            result: 0,
            custom_error: 0,
            modified_accounts: account_outputs,
        },
    };

    let elf = get_test_elf();

    protosol::program_runtime::process_fixture(fixture, elf);
}
