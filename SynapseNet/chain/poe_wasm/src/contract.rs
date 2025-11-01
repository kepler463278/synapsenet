//! PoE contract implementation

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, EpochAcc, Stats, ACCRUAL, CONFIG, EPOCHS, STATS};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        epoch_secs: msg.epoch_secs,
        min_signers: msg.min_signers,
        reward_per_epoch: msg.reward_per_epoch,
    };
    
    CONFIG.save(deps.storage, &config)?;
    STATS.save(deps.storage, &Stats::default())?;
    
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("epoch_secs", msg.epoch_secs.to_string())
        .add_attribute("min_signers", msg.min_signers.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SubmitBatch {
            epoch,
            root,
            participants,
            signers,
            sigs,
        } => execute_submit_batch(deps, env, epoch, root, participants, signers, sigs),
        ExecuteMsg::ClaimReward { node } => execute_claim_reward(deps, info, node),
        ExecuteMsg::UpdateConfig {
            epoch_secs,
            min_signers,
            reward_per_epoch,
        } => execute_update_config(deps, info, epoch_secs, min_signers, reward_per_epoch),
    }
}

fn execute_submit_batch(
    deps: DepsMut,
    _env: Env,
    epoch: u64,
    root: Binary,
    participants: Vec<String>,
    signers: Vec<String>,
    _sigs: Vec<Binary>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Check epoch not already posted
    if EPOCHS.may_load(deps.storage, epoch)?.is_some() {
        return Err(ContractError::EpochAlreadyPosted { epoch });
    }
    
    // Check minimum signers
    if signers.len() < config.min_signers as usize {
        return Err(ContractError::InsufficientSigners {
            required: config.min_signers,
            provided: signers.len() as u32,
        });
    }
    
    // TODO: Verify signatures
    
    // Store epoch
    let epoch_acc = EpochAcc {
        root: root.to_vec(),
        total_weight: cosmwasm_std::Decimal::one(),
        posted: true,
    };
    EPOCHS.save(deps.storage, epoch, &epoch_acc)?;
    
    // Distribute rewards
    let reward_per_participant = config.reward_per_epoch.checked_div(Uint128::from(participants.len() as u128))?;
    
    for participant in &participants {
        let current = ACCRUAL.may_load(deps.storage, participant)?.unwrap_or_default();
        ACCRUAL.save(deps.storage, participant, &(current + reward_per_participant))?;
    }
    
    // Update stats
    let mut stats = STATS.load(deps.storage)?;
    stats.total_epochs += 1;
    stats.total_rewards += config.reward_per_epoch;
    stats.total_participants += participants.len() as u64;
    STATS.save(deps.storage, &stats)?;
    
    Ok(Response::new()
        .add_attribute("action", "submit_batch")
        .add_attribute("epoch", epoch.to_string())
        .add_attribute("participants", participants.len().to_string()))
}

fn execute_claim_reward(
    deps: DepsMut,
    _info: MessageInfo,
    node: String,
) -> Result<Response, ContractError> {
    let amount = ACCRUAL.may_load(deps.storage, &node)?.unwrap_or_default();
    
    if amount.is_zero() {
        return Err(ContractError::NoRewardsToClaim {});
    }
    
    // Clear accrual
    ACCRUAL.save(deps.storage, &node, &Uint128::zero())?;
    
    // TODO: Mint/transfer NGT tokens
    
    Ok(Response::new()
        .add_attribute("action", "claim_reward")
        .add_attribute("node", node)
        .add_attribute("amount", amount.to_string()))
}

fn execute_update_config(
    deps: DepsMut,
    _info: MessageInfo,
    epoch_secs: Option<u64>,
    min_signers: Option<u32>,
    reward_per_epoch: Option<Uint128>,
) -> Result<Response, ContractError> {
    // TODO: Add admin check
    
    let mut config = CONFIG.load(deps.storage)?;
    
    if let Some(secs) = epoch_secs {
        config.epoch_secs = secs;
    }
    if let Some(signers) = min_signers {
        config.min_signers = signers;
    }
    if let Some(reward) = reward_per_epoch {
        config.reward_per_epoch = reward;
    }
    
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new().add_attribute("action", "update_config"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetEpoch { epoch } => to_json_binary(&query_epoch(deps, epoch)?),
        QueryMsg::GetAccrual { node } => to_json_binary(&query_accrual(deps, node)?),
        QueryMsg::GetStats {} => to_json_binary(&query_stats(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

fn query_epoch(deps: Deps, epoch: u64) -> StdResult<Option<EpochAcc>> {
    EPOCHS.may_load(deps.storage, epoch)
}

fn query_accrual(deps: Deps, node: String) -> StdResult<Uint128> {
    Ok(ACCRUAL.may_load(deps.storage, &node)?.unwrap_or_default())
}

fn query_stats(deps: Deps) -> StdResult<Stats> {
    STATS.load(deps.storage)
}
