use crate::state::Post;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterProfileName {
        profile_name: String,
    },
    CreatePost {
        post_title: String,
        external_id: String,
        text: String,
        tags: Vec<String>,
    },
    EditPost {
        post_title: String,
        external_id: String,
        text: String,
        tags: Vec<String>,
    },
    DeletePost {
        post_title: String,
    },
    LikePost {
        post_title: String,
    },
    WithdrawJuno {},
    AdminRegisterProfileName {
        profile_name: String,
        address: String,
    },
    AdminCreatePost {
        post_title: String,
        external_id: String,
        text: String,
        tags: Vec<String>,
        address: String,
        creation: String,
        edit_date: String,
        editor_address: String,
        like_number: u64,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllPostsResponse {
    pub posts: Vec<Post>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PostResponse {
    pub post: Option<Post>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArticleCountResponse {
    pub article_count: u64,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProfileNameResponse {
    pub profile_name: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPosts {
        limit: Option<u32>,
        start_after: Option<u64>,
    },
    Post {
        post_title: String,
    },
    ArticleCount {},
    ProfileName {
        address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}
