//! Just a simple example program for testing the harness's ability to produce
//! a valid program runtime and invoke context.

mod instruction;

use {
    instruction::TestProgramInstruction,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        program::invoke,
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvar::Sysvar,
    },
};

solana_program::declare_id!("239vxAL9Q7e3uLoinJpJ873r3bvT9sPFxH7yekwPppNF");

solana_program::entrypoint!(process_instruction);

fn process_noop(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account1 = next_account_info(accounts_iter)?;
    let account2 = next_account_info(accounts_iter)?;
    if account1.owner != program_id || account2.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

fn process_write_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: [u8; 4],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if account.data_len() != 4 {
        return Err(ProgramError::InvalidAccountData);
    }
    account.try_borrow_mut_data()?.copy_from_slice(&data);
    Ok(())
}

fn process_write_clock_data(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if account.data_len() != 8 {
        return Err(ProgramError::InvalidAccountData);
    }
    let clock = <Clock as Sysvar>::get()?;
    account
        .try_borrow_mut_data()?
        .copy_from_slice(&clock.slot.to_le_bytes());
    Ok(())
}

fn process_close_account(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let destination = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    let new_destination_lamports = account
        .lamports()
        .checked_add(destination.lamports())
        .ok_or(ProgramError::ArithmeticOverflow)?;
    **account.try_borrow_mut_lamports()? = 0;
    **destination.try_borrow_mut_lamports()? = new_destination_lamports;
    account.realloc(0, true)?;
    account.assign(&solana_program::system_program::id());
    Ok(())
}

fn process_transfer(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let sender = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    if sender.owner != program_id || recipient.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !sender.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    **sender.try_borrow_mut_lamports()? = sender
        .lamports()
        .checked_sub(amount)
        .ok_or(ProgramError::InsufficientFunds)?;
    **recipient.try_borrow_mut_lamports()? = recipient
        .lamports()
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    Ok(())
}

fn process_transfer_with_cpi(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let sender = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    invoke(
        &solana_program::system_instruction::transfer(sender.key, recipient.key, amount),
        &[sender.clone(), recipient.clone()],
    )?;
    Ok(())
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = TestProgramInstruction::unpack(input)?;
    match instruction {
        TestProgramInstruction::NoOp => process_noop(program_id, accounts),
        TestProgramInstruction::WriteData { data } => {
            process_write_data(program_id, accounts, data)
        }
        TestProgramInstruction::WriteClockData => process_write_clock_data(program_id, accounts),
        TestProgramInstruction::CloseAccount => process_close_account(program_id, accounts),
        TestProgramInstruction::Transfer { amount } => {
            process_transfer(program_id, accounts, amount)
        }
        TestProgramInstruction::TransferWithCpi { amount } => {
            process_transfer_with_cpi(accounts, amount)
        }
    }
}
