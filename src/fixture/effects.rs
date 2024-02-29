use {
    super::{error::FixtureError, proto},
    solana_sdk::{account::AccountSharedData, pubkey::Pubkey},
};

pub struct FixtureEffects {
    pub result: i32,
    pub custom_error: u64,
    pub modified_accounts: Vec<(Pubkey, AccountSharedData)>,
}

impl TryFrom<proto::InstrEffects> for FixtureEffects {
    type Error = FixtureError;

    fn try_from(input: proto::InstrEffects) -> Result<Self, Self::Error> {
        Ok(Self {
            result: input.result,
            custom_error: input.custom_err,
            modified_accounts: input
                .modified_accounts
                .into_iter()
                .map(|acct_state| acct_state.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
