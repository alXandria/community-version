use cosmwasm_std::{
    coin, entry_point, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use std::{env, vec};

use crate::coin_helpers::assert_sent_exact_coin;
use crate::error::ContractError;
use crate::msg::{
    AllPostsResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, PostResponse, QueryMsg,
};
use crate::state::{Config, Post, CONFIG, LAST_POST_ID, POST};

//info for migration
const CONTRACT_NAME: &str = "crates.io:alxandria";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const ADDRESS: &str = "desmos1a7vpcddckf9qwezmva8gpzphmzhhdr5y2pllvr";
const ADMIN: &str = "desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let validated_admin = deps.api.addr_validate(ADMIN)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePost {
            post_title,
            external_id,
            text,
            tags,
        } => execute_create_post(deps, env, info, post_title, external_id, text, tags),
        ExecuteMsg::EditPost {
            post_id,
            external_id,
            text,
            tags,
        } => execute_edit_post(deps, env, info, post_id, external_id, text, tags),
        ExecuteMsg::DeletePost { post_id } => execute_delete_post(deps, env, info, post_id),
    }
}
//clippy defaults to max value of 7
#[allow(clippy::too_many_arguments)]
fn execute_create_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_title: String,
    external_id: String,
    text: String,
    tags: Vec<String>,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(coin(100_000_000, "udaric")))?;
    //const
    if text.len() > 499 {
        return Err(ContractError::TooMuchText {});
    }
    if external_id.len() > 128 {
        return Err(ContractError::OnlyOneLink {});
    }
    // POST.keys(store, min, max, order)
    //load descending, last instead of last_post_id
    let last_post_id = LAST_POST_ID.may_load(deps.storage)?;
    match last_post_id {
        Some(last_post_id) => {
            let incremented_id = last_post_id + 1;
            let author = info.sender.to_string();
            let validated_author = deps.api.addr_validate(&author)?;
            let post: Post = Post {
                post_id: incremented_id,
                post_title,
                external_id,
                text,
                tags,
                author: validated_author.to_string(),
                creation_date: env.block.time.to_string(),
                last_edit_date: None,
                deleter: None,
                editor: None,
            };
            LAST_POST_ID.save(deps.storage, &incremented_id)?;
            POST.save(deps.storage, post.post_id, &post)?;
            let message = BankMsg::Send {
                to_address: (ADDRESS.to_string()),
                amount: vec![coin(100_000_000, "udaric")],
            };
            Ok(Response::new()
                .add_message(message)
                .add_attribute("action", "create_post")
                .add_attribute("post_id", post.post_id.to_string())
                .add_attribute("author", validated_author.to_string()))
        }
        None => {
            let last_post_id = 0;
            let incremented_id = last_post_id + 1;
            let author = info.sender.to_string();
            let validated_author = deps.api.addr_validate(&author)?;
            let post: Post = Post {
                post_id: incremented_id,
                post_title,
                external_id,
                text,
                tags,
                author: validated_author.to_string(),
                creation_date: env.block.time.to_string(),
                last_edit_date: None,
                deleter: None,
                editor: None,
            };
            LAST_POST_ID.save(deps.storage, &incremented_id)?;
            POST.save(deps.storage, post.post_id, &post)?;
            let message = BankMsg::Send {
                to_address: (ADDRESS.to_string()),
                amount: vec![coin(100_000_000, "udaric")],
            };
            Ok(Response::new()
                .add_message(message)
                .add_attribute("action", "create_post")
                .add_attribute("post_id", post.post_id.to_string())
                .add_attribute("author", validated_author.to_string()))
        }
    }
}

fn execute_edit_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_id: u64,
    external_id: String,
    text: String,
    tags: Vec<String>,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(Coin::new(200_000_000, "udaric")))?;
    if text.len() > 499 {
        return Err(ContractError::TooMuchText {});
    }
    if external_id.len() > 128 {
        return Err(ContractError::OnlyOneLink {});
    }
    let post = POST.load(deps.storage, post_id)?;
    let editor = info.sender.to_string();
    let validated_editor = deps.api.addr_validate(&editor)?;
    let new_post: Post = Post {
        post_id: post.post_id,
        post_title: post.post_title,
        external_id,
        text,
        tags,
        author: post.author,
        creation_date: post.creation_date,
        last_edit_date: Some(env.block.time.to_string()),
        deleter: None,
        editor: Some(validated_editor.to_string()),
    };
    POST.save(deps.storage, post_id, &new_post)?;
    let message = BankMsg::Send {
        to_address: (ADDRESS.to_string()),
        amount: vec![coin(200_000_000, "udaric")],
    };
    Ok(Response::new()
        .add_message(message)
        .add_attribute("action", "edit_post")
        .add_attribute("post_id", new_post.post_id.to_string())
        .add_attribute("editor", new_post.editor.unwrap()))
}
fn execute_delete_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_id: u64,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(Coin::new(1_000_000_000, "udaric")))?;
    let post = POST.load(deps.storage, post_id)?;
    let deleter = info.sender.to_string();
    let validated_deleter = deps.api.addr_validate(&deleter)?;
    let deleted_post: Post = Post {
        post_id: post.post_id,
        post_title: post.post_title,
        external_id: "".to_string(),
        text: "This post has been deleted.".to_string(),
        tags: vec!["Deleted".to_string()],
        author: post.author,
        creation_date: post.creation_date,
        last_edit_date: Some(env.block.time.to_string()),
        deleter: Some(validated_deleter.to_string()),
        editor: post.editor,
    };
    POST.save(deps.storage, post_id, &deleted_post)?;
    let message = BankMsg::Send {
        to_address: (ADDRESS.to_string()),
        amount: vec![coin(1_000_000_000, "udaric")],
    };
    Ok(Response::new()
        .add_message(message)
        .add_attribute("action", "delete_post")
        .add_attribute("post_id", deleted_post.post_id.to_string())
        .add_attribute("delete", deleted_post.deleter.unwrap()))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPosts { limit } => query_all_posts(deps, env, limit),
        QueryMsg::Post { post_id } => query_post(deps, env, post_id),
    }
}

//pagination limits

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

fn query_all_posts(deps: Deps, _env: Env, limit: Option<u32>) -> StdResult<Binary> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let posts = POST
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllPostsResponse { posts })
}

fn query_post(deps: Deps, _env: Env, post_id: u64) -> StdResult<Binary> {
    let post = POST.may_load(deps.storage, post_id)?;
    to_binary(&PostResponse { post })
}

#[entry_point]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    let ver = get_contract_version(deps.storage)?;
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default()
        .add_attribute("action", "migration")
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("contract", CONTRACT_NAME))
}
