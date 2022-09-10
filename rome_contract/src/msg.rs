use cosmwasm_std::{Uint64, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use desmos_bindings::types::PageRequest;
use desmos_bindings::posts::models::{Entities, RawPostAttachment, ReplySetting, PostReference};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePost{
        subspace_id: Uint64,
        section_id: u32,
        external_id: Option<String>,
        text: Option<String>,
        entities: Option<Entities>,
        attachments: Option<Vec<RawPostAttachment>>,
        author: Addr,
        conversation_id: Option<Uint64>,
        reply_settings: ReplySetting,
        referenced_posts: Vec<PostReference>,
    },
    EditPost{
        subspace_id: Uint64,
        post_id: Uint64,
        text: String,
        entities: Option<Entities>,
        editor: Addr,
    },
    DeletePost{
        subspace_id: Uint64,
        post_id: Uint64,
        signer: Addr,
    },
    AddPostAttachment{
        subspace_id: Uint64,
        post_id: Uint64,
        content: RawPostAttachment,
        editor: Addr,
    },
    RemovePostAttachment{
        subspace_id: Uint64,
        post_id: Uint64,
        attachement_id: u32,
        editor: Addr,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    SubspacePosts{
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    },
    SectionPosts{
        subspace_id: Uint64,
        section_id: u32,
        pagination: Option<PageRequest>,
    },
    Post{
        
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
