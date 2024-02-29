//! Just a simple example program for testing the harness's ability to produce
//! a valid program runtime and invoke context.

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

solana_program::declare_id!("239vxAL9Q7e3uLoinJpJ873r3bvT9sPFxH7yekwPppNF");

solana_program::entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    match input {
        &[1, 2, 3, 4] => Ok(()),
        _ => Err(ProgramError::Custom(220_220_220)),
    }?;

    let accounts_iter = &mut accounts.iter();
    let account1 = next_account_info(accounts_iter)?;
    let account2 = next_account_info(accounts_iter)?;

    let new_destination_lamports = account1
        .lamports()
        .checked_add(account2.lamports())
        .ok_or(ProgramError::ArithmeticOverflow)?;

    **account1.try_borrow_mut_lamports()? = 0;
    **account2.try_borrow_mut_lamports()? = new_destination_lamports;

    Ok(())
}
