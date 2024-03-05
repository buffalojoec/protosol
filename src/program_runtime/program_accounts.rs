//! Program accounts.

use solana_sdk::{
    account::{Account, AccountSharedData},
    bpf_loader_upgradeable::UpgradeableLoaderState,
    pubkey::Pubkey,
    rent::Rent,
};

/// Create the program accounts for a given program.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_accounts() {
        let program_id = Pubkey::new_unique();
        let rent = Rent::default();
        let elf = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Non-upgradeable loader

        let loader_id = solana_sdk::bpf_loader::id();

        let accounts = program_accounts(&program_id, &loader_id, &rent, &elf);
        assert_eq!(accounts.len(), 1);

        let (result_program_address, result_program_account) = &accounts[0];
        assert_eq!(result_program_address, &program_id);
        assert_eq!(
            result_program_account,
            &AccountSharedData::from(Account {
                lamports: rent.minimum_balance(elf.len()).max(1),
                data: elf.clone(),
                owner: loader_id,
                executable: true,
                rent_epoch: 0,
            })
        );

        // Upgradeable loader

        let loader_id = solana_sdk::bpf_loader_upgradeable::id();
        let programdata_address =
            Pubkey::find_program_address(&[program_id.as_ref()], &loader_id).0;

        let data = bincode::serialize(&UpgradeableLoaderState::Program {
            programdata_address,
        })
        .unwrap();
        let mut program_data = bincode::serialize(&UpgradeableLoaderState::ProgramData {
            slot: 0,
            upgrade_authority_address: Some(Pubkey::default()),
        })
        .unwrap();
        program_data.extend_from_slice(&elf);

        let accounts = program_accounts(&program_id, &loader_id, &rent, &elf);
        assert_eq!(accounts.len(), 2);

        let (result_program_address, result_program_account) = &accounts[0];
        assert_eq!(result_program_address, &program_id);
        assert_eq!(
            result_program_account,
            &AccountSharedData::from(Account {
                lamports: rent.minimum_balance(data.len()).max(1),
                data,
                owner: loader_id,
                executable: true,
                rent_epoch: 0,
            })
        );

        let (result_programdata_address, result_programdata_account) = &accounts[1];
        assert_eq!(result_programdata_address, &programdata_address);
        assert_eq!(
            result_programdata_account,
            &AccountSharedData::from(Account {
                lamports: rent.minimum_balance(program_data.len()).max(1),
                data: program_data,
                owner: loader_id,
                executable: false,
                rent_epoch: 0,
            })
        );
    }
}
