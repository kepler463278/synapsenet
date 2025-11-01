//! PoE contract messages

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub epoch_secs: u64,
    pub min_signers: u32,
    pub reward_per_epoch: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    SubmitBatch {
        epoch: u64,
        root: Binary,
        participants: Vec<String>,
        signers: Vec<String>,
        sigs: Vec<Binary>,
    },
    ClaimReward {
        node: String,
    },
    UpdateConfig {
        epoch_secs: Option<u64>,
        min_signers: Option<u32>,
        reward_per_epoch: Option<Uint128>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
    
    #[returns(EpochResponse)]
    GetEpoch { epoch: u64 },
    
    #[returns(AccrualResponse)]
    GetAccrual { node: String },
    
    #[returns(StatsResponse)]
    GetStats {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub epoch_secs: u64,
    pub min_signers: u32,
    pub reward_per_epoch: Uint128,
}

#[cw_serde]
pub struct EpochResponse {
    pub epoch: u64,
    pub root: Binary,
    pub total_weight: String,
    pub posted: bool,
}

#[cw_serde]
pub struct AccrualResponse {
    pub node: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct StatsResponse {
    pub total_epochs: u64,
    pub total_rewards: Uint128,
    pub total_participants: u64,
}
