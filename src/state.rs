use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr
}

// Derive JSON serialization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}

// Derive JSON serialization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
// String - UUID generated clientside
pub const POLLS: Map<String, Poll> = Map::new("polls");
// (Addr - address of voter, String - Poll UUID)
pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");

