use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint64};
use desmos_bindings::posts::models::{Entities, PostReference, ReplySetting};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Post {
    pub post_id: u64,
    pub subspace_id: Uint64,
    pub section_id: u32,
    pub external_id: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<Entities>>,
    pub tags: Vec<String>,
    pub author: Addr,
    pub conversation_id: Option<Uint64>,
    pub referenced_posts: Vec<PostReference>,
    pub reply_settings: ReplySetting,
    pub creation_date: String,
    pub last_edit_date: Option<String>,
}

pub const CONFIG: Item<Config> = Item::new("config");
//create a map of post. Uint64 is post_id
pub const POST: Map<u64, Post> = Map::new("post");
