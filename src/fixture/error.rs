use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, PartialEq)]
pub enum FixtureError {
    #[error("Invalid protobuf")]
    InvalidProtobuf(#[from] prost::DecodeError),
    #[error("Integer out of range")]
    IntegerOutOfRange,
    #[error("Invalid hash bytes")]
    InvalidHashBytes,
    #[error("Invalid public key bytes")]
    InvalidPubkeyBytes,
    #[error("Account missing")]
    AccountMissing,
    #[error("Invalid fixture input")]
    InvalidFixtureInput,
    #[error("Invalid fixture output")]
    InvalidFixtureOutput,
    #[error("Failed to initialize loader")]
    FailedToInitializeLoader,
}
