use solana_sdk::{
    account::{Account, AccountSharedData},
    bpf_loader_upgradeable::UpgradeableLoaderState,
    pubkey::Pubkey,
    rent::Rent,
};

pub fn program_accounts(
    program_id: &Pubkey,
    loader_id: &Pubkey,
    rent: &Rent,
    elf: &[u8],
) -> Vec<(Pubkey, AccountSharedData)> {
    let mut accounts = vec![];

    let data = if *loader_id == solana_sdk::bpf_loader_upgradeable::ID {
        let (programdata_address, _) =
            Pubkey::find_program_address(&[program_id.as_ref()], loader_id);

        let mut program_data = bincode::serialize(&UpgradeableLoaderState::ProgramData {
            slot: 0,
            upgrade_authority_address: Some(Pubkey::default()),
        })
        .unwrap();
        program_data.extend_from_slice(elf);

        accounts.push((
            programdata_address,
            AccountSharedData::from(Account {
                lamports: rent.minimum_balance(program_data.len()).max(1),
                data: program_data,
                owner: *loader_id,
                executable: false,
                rent_epoch: 0,
            }),
        ));

        bincode::serialize(&UpgradeableLoaderState::Program {
            programdata_address,
        })
        .unwrap()
    } else {
        elf.to_vec()
    };

    accounts.push((
        *program_id,
        AccountSharedData::from(Account {
            lamports: rent.minimum_balance(data.len()).max(1),
            data,
            owner: *loader_id,
            executable: true,
            rent_epoch: 0,
        }),
    ));

    accounts.reverse();
    accounts
}
