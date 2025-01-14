use cosmwasm_std::{to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Uint128};
use cw_storage_plus::Map;
use crate::error::ContractError;
use crate::message::{InstantiateMsg, ExecuteMsg, QueryMsg, Creator};

pub const CREATORS: Map<String, Creator> = Map::new("creators");
pub const SUBSCRIPTION_FEE: Map<&str, Uint128> = Map::new("subscription_fee");
pub const SUBSCRIPTION_DENOM: Map<&str, String> = Map::new("subscription_denom");

// --- Instantiate ---
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    SUBSCRIPTION_FEE.save(deps.storage, "fee", &msg.subscription_fee)?;
    SUBSCRIPTION_DENOM.save(deps.storage, "denom", &msg.subscription_denom)?;
    Ok(Response::default())
}

// --- Execute ---
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterCreator { name } => register_creator(deps, info, name),
        ExecuteMsg::Subscribe { creator } => subscribe_to_creator(deps, info, creator),
        ExecuteMsg::PublishContent { content } => publish_content(deps, info, content),
    }
}

fn register_creator(deps: DepsMut, info: MessageInfo, name: String) -> Result<Response, ContractError> {
    let creator = Creator {
        name,
        subscribers: vec![],
        content: vec![],
    };
    CREATORS.save(deps.storage, info.sender.to_string(), &creator)?;
    Ok(Response::new().add_attribute("action", "register_creator"))
}

fn subscribe_to_creator(
    deps: DepsMut,
    info: MessageInfo,
    creator: String,
) -> Result<Response, ContractError> {
    let subscription_fee = SUBSCRIPTION_FEE.load(deps.storage, "fee")?;
    let subscription_denom = SUBSCRIPTION_DENOM.load(deps.storage, "denom")?;

    // Validate payment
    let payment = info.funds.iter().find(|c| c.denom == subscription_denom);
    if let Some(coin) = payment {
        if coin.amount < subscription_fee {
            return Err(ContractError::InsufficientFunds);
        }
    } else {
        return Err(ContractError::NoFundsSent);
    }

    let mut creator_data = CREATORS.load(deps.storage, creator.clone())?;
    if !creator_data.subscribers.contains(&info.sender.to_string()) {
        creator_data.subscribers.push(info.sender.to_string());
    }
    CREATORS.save(deps.storage, creator.clone(), &creator_data)?;

    // Transfer funds to creator
    let bank_msg = BankMsg::Send {
        to_address: creator,
        amount: vec![Coin {
            denom: subscription_denom,
            amount: subscription_fee,
        }],
    };

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "subscribe"))
}

fn publish_content(deps: DepsMut, info: MessageInfo, content: String) -> Result<Response, ContractError> {
    let mut creator_data = CREATORS.load(deps.storage, info.sender.to_string())?;
    creator_data.content.push(content);
    CREATORS.save(deps.storage, info.sender.to_string(), &creator_data)?;
    Ok(Response::new().add_attribute("action", "publish_content"))
}

// --- Query ---
pub fn query(deps: Deps, _env: Env, info: MessageInfo, msg: QueryMsg) -> Result<Vec<String>, ContractError> {
    match msg {
        QueryMsg::GetContent { creator } => get_content(deps, creator, info.sender.to_string()),
    }
}

fn get_content(deps: Deps, creator: String, requester: String) -> Result<Vec<String>, ContractError> {
    let creator_data = CREATORS.load(deps.storage, creator.clone())?;
    if !creator_data.subscribers.contains(&requester) {
        return Err(ContractError::Unauthorized);
    }
    Ok(creator_data.content)
}

// Tests and error handling would be added here for production readiness.
