use {
    super::{error::FixtureError, proto},
    solana_program_runtime::sysvar_cache::SysvarCache,
    solana_sdk::{
        clock::Clock,
        epoch_rewards::EpochRewards,
        epoch_schedule::EpochSchedule,
        hash::Hash,
        rent::Rent,
        slot_hashes::{SlotHash, SlotHashes},
        stake_history::{StakeHistory, StakeHistoryEntry},
    },
};

impl From<proto::Clock> for Clock {
    fn from(input: proto::Clock) -> Self {
        Self {
            slot: input.slot,
            epoch_start_timestamp: input.epoch_start_timestamp,
            epoch: input.epoch,
            leader_schedule_epoch: input.leader_schedule_epoch,
            unix_timestamp: input.unix_timestamp,
        }
    }
}

impl From<proto::EpochRewards> for EpochRewards {
    fn from(input: proto::EpochRewards) -> Self {
        Self {
            total_rewards: input.total_rewards,
            distributed_rewards: input.distributed_rewards,
            distribution_complete_block_height: input.distribution_complete_block_height,
        }
    }
}

impl From<proto::EpochSchedule> for EpochSchedule {
    fn from(input: proto::EpochSchedule) -> Self {
        Self {
            slots_per_epoch: input.slots_per_epoch,
            leader_schedule_slot_offset: input.leader_schedule_slot_offset,
            warmup: input.warmup,
            first_normal_epoch: input.first_normal_epoch,
            first_normal_slot: input.first_normal_slot,
        }
    }
}

impl TryFrom<proto::Rent> for Rent {
    type Error = FixtureError;

    fn try_from(input: proto::Rent) -> Result<Self, Self::Error> {
        let burn_percent =
            u8::try_from(input.burn_percent).map_err(|_| FixtureError::IntegerOutOfRange)?;
        Ok(Rent {
            lamports_per_byte_year: input.lamports_per_byte_year,
            exemption_threshold: input.exemption_threshold,
            burn_percent,
        })
    }
}

impl TryFrom<proto::SlotHashEntry> for SlotHash {
    type Error = FixtureError;

    fn try_from(input: proto::SlotHashEntry) -> Result<Self, Self::Error> {
        let hash = Hash::new_from_array(
            input
                .hash
                .try_into()
                .map_err(|_| FixtureError::InvalidHashBytes)?,
        );
        Ok((input.slot, hash))
    }
}

impl TryFrom<proto::SlotHashes> for SlotHashes {
    type Error = FixtureError;

    fn try_from(input: proto::SlotHashes) -> Result<Self, Self::Error> {
        let slot_hashes: Vec<SlotHash> = input
            .slot_hashes
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(SlotHashes::new(&slot_hashes))
    }
}

impl From<proto::StakeHistoryEntry> for (u64, StakeHistoryEntry) {
    fn from(input: proto::StakeHistoryEntry) -> (u64, StakeHistoryEntry) {
        (
            input.epoch,
            StakeHistoryEntry {
                effective: input.effective,
                activating: input.activating,
                deactivating: input.deactivating,
            },
        )
    }
}

impl From<proto::StakeHistory> for StakeHistory {
    fn from(input: proto::StakeHistory) -> Self {
        let mut stake_history = StakeHistory::default();
        for (epoch, entry) in input.stake_history.into_iter().map(Into::into) {
            stake_history.add(epoch, entry);
        }
        stake_history
    }
}

#[derive(Default)]
pub struct FixtureSysvarContext {
    pub clock: Clock,
    pub epoch_rewards: EpochRewards,
    pub epoch_schedule: EpochSchedule,
    pub rent: Rent,
    pub slot_hashes: SlotHashes,
    pub stake_history: StakeHistory,
}

impl TryFrom<proto::SysvarContext> for FixtureSysvarContext {
    type Error = FixtureError;

    fn try_from(input: proto::SysvarContext) -> Result<Self, Self::Error> {
        Ok(Self {
            clock: input.clock.map(Into::into).unwrap_or_default(),
            epoch_rewards: input.epoch_rewards.map(Into::into).unwrap_or_default(),
            epoch_schedule: input.epoch_schedule.map(Into::into).unwrap_or_default(),
            rent: input
                .rent
                .map(TryInto::try_into)
                .transpose()?
                .unwrap_or_default(),
            slot_hashes: input
                .slot_hashes
                .map(TryInto::try_into)
                .transpose()?
                .unwrap_or_default(),
            stake_history: input.stake_history.map(Into::into).unwrap_or_default(),
        })
    }
}

impl From<FixtureSysvarContext> for SysvarCache {
    fn from(input: FixtureSysvarContext) -> Self {
        let FixtureSysvarContext {
            clock,
            epoch_rewards,
            epoch_schedule,
            rent,
            slot_hashes,
            stake_history,
        } = input;
        let mut sysvar_cache = SysvarCache::default();
        sysvar_cache.set_clock(clock);
        sysvar_cache.set_epoch_rewards(epoch_rewards);
        sysvar_cache.set_epoch_schedule(epoch_schedule);
        sysvar_cache.set_rent(rent);
        sysvar_cache.set_slot_hashes(slot_hashes);
        sysvar_cache.set_stake_history(stake_history);
        sysvar_cache
    }
}
