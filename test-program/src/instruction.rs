use solana_program::program_error::ProgramError;

pub enum TestProgramInstruction {
    /// No-op. Do nothing.
    /// 0. `[]` Some program-owned account.
    /// 1. `[]` Another program-owned account.
    NoOp,
    /// Write some data to an account. The account is expected to be
    /// initialized.
    /// 0. `[writable]` The program-owned account to write to.
    WriteData { data: [u8; 4] },
    /// Write the clock's current slot to an account. The account is expected
    /// to be initialized.
    /// 0. `[writable]` The program-owned account to write to.
    WriteClockData,
    /// Close an account by transferring all its lamports to the destination
    /// account and clearing its data.
    /// 0. `[writable]` The account to close.
    /// 1. `[writable]` The destination account.
    CloseAccount,
    /// Transfer lamports from one account to another.
    /// 0. `[writable]` The program-owned sender.
    /// 1. `[writable]` The program-owned recipient.
    Transfer { amount: u64 },
    /// Transfer lamports from one account to another.
    /// 0. `[writable]` The system account sender.
    /// 1. `[writable]` The system account recipient.
    /// 2. `[]`         The System program.
    TransferWithCpi { amount: u64 },
}

impl TestProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => Self::NoOp,
            1 => {
                if rest.len() < 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let data = rest[..4].try_into().unwrap();
                Self::WriteData { data }
            }
            2 => Self::WriteClockData,
            3 => Self::CloseAccount,
            4 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                Self::Transfer { amount }
            }
            5 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                Self::TransferWithCpi { amount }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
