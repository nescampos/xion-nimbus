use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use serde::{Deserialize, Serialize};
use cw_storage_plus::Map;

// --- Messages ---
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterCreator { name: String },
    Subscribe { creator: String },
    PublishContent { content: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetContent { creator: String },
}

// --- State ---
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Creator {
    pub name: String,
    pub subscribers: Vec<String>,
    pub content: Vec<String>,
}

pub const CREATORS: Map<String, Creator> = Map::new("creators");

// --- Instantiate ---
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

// --- Execute ---
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::RegisterCreator { name } => register_creator(deps, info, name),
        ExecuteMsg::Subscribe { creator } => subscribe_to_creator(deps, info, creator),
        ExecuteMsg::PublishContent { content } => publish_content(deps, info, content),
    }
}

fn register_creator(deps: DepsMut, info: MessageInfo, name: String) -> StdResult<Response> {
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
) -> StdResult<Response> {
    let mut creator_data = CREATORS.load(deps.storage, creator.clone())?;
    if !creator_data.subscribers.contains(&info.sender.to_string()) {
        creator_data.subscribers.push(info.sender.to_string());
    }
    CREATORS.save(deps.storage, creator, &creator_data)?;
    Ok(Response::new().add_attribute("action", "subscribe"))
}

fn publish_content(deps: DepsMut, info: MessageInfo, content: String) -> StdResult<Response> {
    let mut creator_data = CREATORS.load(deps.storage, info.sender.to_string())?;
    creator_data.content.push(content);
    CREATORS.save(deps.storage, info.sender.to_string(), &creator_data)?;
    Ok(Response::new().add_attribute("action", "publish_content"))
}

// --- Query ---
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContent { creator } => to_binary(&get_content(deps, creator, _env.message.sender.to_string())?),
    }
}

fn get_content(deps: Deps, creator: String, requester: String) -> StdResult<Vec<String>> {
    let creator_data = CREATORS.load(deps.storage, creator.clone())?;
    if !creator_data.subscribers.contains(&requester) {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized: Not a subscriber"));
    }
    Ok(creator_data.content)
}
