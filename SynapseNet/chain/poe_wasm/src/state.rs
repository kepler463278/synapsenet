//! PoE contract state

use cosmwasm_std::{Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub epoch_secs: u64,
    pub min_signers: u32,
    pub reward_per_epoch: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EpochAcc {
    pub root: Vec<u8>,
    pub total_weight: Decimal,
    pub posted: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const EPOCHS: Map<u64, EpochAcc> = Map::new("epochs");
pub const ACCRUAL: Map<&str, Uint128> = Map::new("accrual");
pub const STATS: Item<Stats> = Item::new("stats");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Stats {
    pub total_epochs: u64,
    pub total_rewards: Uint128,
    pub total_participants: u64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            total_epochs: 0,
            total_rewards: Uint128::zero(),
            total_participants: 0,
        }
    }
}
