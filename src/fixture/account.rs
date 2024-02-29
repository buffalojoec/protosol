use {
    super::{error::FixtureError, proto},
    solana_sdk::{
        account::{Account, AccountSharedData},
        pubkey::Pubkey,
    },
};

impl TryFrom<proto::AcctState> for (Pubkey, AccountSharedData) {
    type Error = FixtureError;

    fn try_from(input: proto::AcctState) -> Result<Self, Self::Error> {
        let pubkey = Pubkey::new_from_array(
            input
                .address
                .try_into()
                .map_err(|_| FixtureError::InvalidPubkeyBytes)?,
        );
        let owner = Pubkey::new_from_array(
            input
                .owner
                .try_into()
                .map_err(|_| FixtureError::InvalidPubkeyBytes)?,
        );

        let account = AccountSharedData::from(Account {
            lamports: input.lamports,
            data: input.data,
            owner,
            executable: input.executable,
            rent_epoch: input.rent_epoch,
        });

        Ok((pubkey, account))
    }
}
