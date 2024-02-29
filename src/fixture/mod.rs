pub mod account;
pub mod context;
pub mod effects;
pub mod error;
pub mod feature_set;
mod proto {
    include!(concat!(env!("OUT_DIR"), "/org.solana.compat.program.rs"));
}
pub mod sysvars;

use {context::FixtureContext, effects::FixtureEffects, error::FixtureError, prost::Message};

pub struct Fixture {
    pub input: FixtureContext,
    pub output: FixtureEffects,
}

impl TryFrom<proto::InstrFixture> for Fixture {
    type Error = FixtureError;

    fn try_from(fixture: proto::InstrFixture) -> Result<Self, Self::Error> {
        // All blobs should have an input and output.
        let input: FixtureContext = fixture
            .input
            .ok_or::<FixtureError>(FixtureError::InvalidFixtureInput)?
            .try_into()?;
        let output: FixtureEffects = fixture
            .output
            .ok_or::<FixtureError>(FixtureError::InvalidFixtureOutput)?
            .try_into()?;
        Ok(Self { input, output })
    }
}

impl Fixture {
    pub fn decode(blob: &[u8]) -> Result<Self, FixtureError> {
        let fixture: proto::InstrFixture = proto::InstrFixture::decode(blob)?;
        fixture.try_into()
    }
}
