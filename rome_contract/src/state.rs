use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Post {
    pub post_id: u64,
    pub external_id: String,
    pub text: Option<String>,
    pub tags: Vec<String>,
    pub author: String,
    pub creation_date: String,
    pub last_edit_date: Option<String>,
    pub deleter: Option<String>
}

pub const CONFIG: Item<Config> = Item::new("config");
//create a map of post. Addr is creator. u64 is post_id
pub const POST: Map<u64, Post> = Map::new("post");
