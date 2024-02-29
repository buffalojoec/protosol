use {
    super::{error::FixtureError, proto, sysvars::FixtureSysvarContext},
    solana_sdk::{
        account::AccountSharedData, feature_set::FeatureSet, instruction::AccountMeta,
        pubkey::Pubkey,
    },
};

pub struct FixtureContext {
    pub program_id: Pubkey,
    pub loader_id: Pubkey,
    pub feature_set: FeatureSet,
    pub sysvar_context: FixtureSysvarContext,
    pub accounts: Vec<(Pubkey, AccountSharedData)>,
    pub instruction_accounts: Vec<AccountMeta>,
    pub instruction_data: Vec<u8>,
}

impl TryFrom<proto::InstrContext> for FixtureContext {
    type Error = FixtureError;

    fn try_from(input: proto::InstrContext) -> Result<Self, Self::Error> {
        let program_id = Pubkey::new_from_array(
            input
                .program_id
                .try_into()
                .map_err(|_| FixtureError::InvalidPubkeyBytes)?,
        );
        let loader_id = Pubkey::new_from_array(
            input
                .loader_id
                .try_into()
                .map_err(|_| FixtureError::InvalidPubkeyBytes)?,
        );

        let feature_set = input.feature_set.map(|fs| fs.into()).unwrap_or_default();

        let sysvar_context = input
            .sysvars
            .map(|sysvars| sysvars.try_into())
            .transpose()?
            .unwrap_or_default();

        let accounts = input
            .accounts
            .into_iter()
            .map(|acct_state| acct_state.try_into())
            .collect::<Result<Vec<(Pubkey, AccountSharedData)>, FixtureError>>()?;

        let instruction_accounts = input
            .instr_accounts
            .into_iter()
            .map(
                |proto::InstrAcct {
                     index,
                     is_signer,
                     is_writable,
                 }| {
                    accounts
                        .get(index as usize)
                        .ok_or(FixtureError::AccountMissing)
                        .map(|(pubkey, _)| AccountMeta {
                            pubkey: *pubkey,
                            is_signer,
                            is_writable,
                        })
                },
            )
            .collect::<Result<Vec<_>, _>>()?;

        let instruction_data = input.data;

        Ok(Self {
            program_id,
            loader_id,
            feature_set,
            sysvar_context,
            accounts,
            instruction_accounts,
            instruction_data,
        })
    }
}
