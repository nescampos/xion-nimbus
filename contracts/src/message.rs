use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128};

// --- Messages ---
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub subscription_fee: Uint128,
    pub subscription_denom: String,
}

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