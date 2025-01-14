use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, StdResult, Response, Deps, Binary};
use crate::message::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::error::ContractError;

mod error;
mod contract;
mod message;


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps,_env,_info,msg)
}

// --- Execute ---
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    contract::execute(deps,_env,info,msg)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env,info: MessageInfo, msg: QueryMsg) -> Result<Vec<String>, ContractError> {
    contract::query(deps,_env, info,msg)
}