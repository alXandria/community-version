//profile name query

use cosmwasm_std::{
    coin, entry_point, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use cw_storage_plus::Bound;
use is_false::is_false;
use std::{env, vec};

use crate::coin_helpers::assert_sent_exact_coin;
use crate::error::ContractError;
use crate::msg::{
    AllPostsResponse, ArticleCountResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, PostResponse,
    ProfileNameResponse, QueryMsg,
};
use crate::state::{
    Config, Post, ARTICLE_COUNT, CONFIG, LAST_POST_ID, POST, PROFILE_NAME, REVERSE_LOOKUP,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
//Withdraw address
const ADDRESS: &str = "juno1ggtuwvungvx5t3awqpcqvxxvgt7gvwdkanuwtm";
const ADMIN: &str = "juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4";
const MAX_ID_LENGTH: usize = 128;
const MAX_TEXT_LENGTH: usize = 499;
const IPFS: &str = "https://alxandria.infura-ipfs.io/ipfs/";
const JUNO: &str = "ujunox";

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
    LAST_POST_ID.save(deps.storage, &0)?;
    ARTICLE_COUNT.save(deps.storage, &0)?;
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
        ExecuteMsg::RegisterProfileName { profile_name } => {
            execute_register_profile_name(deps, env, info, profile_name)
        }
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
        ExecuteMsg::LikePost { post_id } => execute_like_post(deps, env, info, post_id),
        ExecuteMsg::Withdraw {} => execute_withdraw(deps, env, info),
    }
}
fn execute_register_profile_name(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    profile_name: String,
) -> Result<Response, ContractError> {
    #[allow(clippy::single_char_pattern)]
    let formatted_profile_name = profile_name.trim().to_lowercase().replace(" ", "");
    //1) Check to see if there is the desired profile name is registered
    let check = REVERSE_LOOKUP.may_load(deps.storage, formatted_profile_name.clone())?;
    match check {
        Some(_check) => Err(ContractError::ProfileNameTaken {
            taken_profile_name: formatted_profile_name,
        }),
        //2) If profile name isn't registered, save it to account
        None => {
            PROFILE_NAME.save(deps.storage, info.sender.clone(), &formatted_profile_name)?;
            REVERSE_LOOKUP.save(deps.storage, formatted_profile_name.clone(), &info.sender)?;
            Ok(Response::new()
                .add_attribute("action", "create profile name")
                .add_attribute("new profile name", formatted_profile_name))
        }
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
    // assert_sent_exact_coin(&info.funds, Some(coin(1_000_000, JUNO)))?;
    if text.len() > MAX_TEXT_LENGTH {
        return Err(ContractError::TooMuchText {});
    }
    if external_id.len() > MAX_ID_LENGTH {
        return Err(ContractError::OnlyOneLink {});
    }
    if is_false(external_id.starts_with(IPFS)) {
        return Err(ContractError::MustUseAlxandriaGateway {});
    }
    let counter = ARTICLE_COUNT.load(deps.storage)?;
    let updated_counter = counter + 1;
    let last_post_id = LAST_POST_ID.load(deps.storage)?;
    let incremented_id = last_post_id + 1;
    let load = PROFILE_NAME.may_load(deps.storage, info.sender.clone())?;
    match load {
        Some(load) => {
            let post: Post = Post {
                post_id: incremented_id,
                post_title,
                external_id,
                text,
                tags,
                author: load.clone(),
                creation_date: env.block.time.to_string(),
                last_edit_date: None,
                editor: None,
                likes: 0,
            };
            LAST_POST_ID.save(deps.storage, &incremented_id)?;
            POST.save(deps.storage, post.post_id, &post)?;
            ARTICLE_COUNT.save(deps.storage, &updated_counter)?;
            Ok(Response::new()
                .add_attribute("action", "create_post")
                .add_attribute("post_id", post.post_id.to_string())
                .add_attribute("author", load))
        }
        None => {
            let post: Post = Post {
                post_id: incremented_id,
                post_title,
                external_id,
                text,
                tags,
                author: info.sender.to_string(),
                creation_date: env.block.time.to_string(),
                last_edit_date: None,
                editor: None,
                likes: 0,
            };
            LAST_POST_ID.save(deps.storage, &incremented_id)?;
            POST.save(deps.storage, post.post_id, &post)?;
            ARTICLE_COUNT.save(deps.storage, &updated_counter)?;
            Ok(Response::new()
                .add_attribute("action", "create_post")
                .add_attribute("post_id", post.post_id.to_string())
                .add_attribute("author", info.sender.to_string()))
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
    assert_sent_exact_coin(&info.funds, Some(Coin::new(200_000, JUNO)))?;
    if text.len() > MAX_TEXT_LENGTH {
        return Err(ContractError::TooMuchText {});
    }
    if external_id.len() > MAX_ID_LENGTH {
        return Err(ContractError::OnlyOneLink {});
    }
    if is_false(external_id.starts_with(IPFS)) {
        return Err(ContractError::MustUseAlxandriaGateway {});
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
        editor: Some(validated_editor.to_string()),
        likes: post.likes,
    };
    POST.save(deps.storage, post_id, &new_post)?;
    let share_address = REVERSE_LOOKUP.may_load(deps.storage, new_post.author.clone())?;
    match share_address {
        Some(share_address) => {
            let share = BankMsg::Send {
                to_address: share_address.to_string(),
                amount: vec![coin(500_000, JUNO)],
            };
            Ok(Response::new()
                .add_message(share)
                .add_attribute("action", "edit_post")
                .add_attribute("post_id", new_post.post_id.to_string())
                .add_attribute("editor", new_post.editor.unwrap()))
        }
        None => {
            let share = BankMsg::Send {
                to_address: new_post.author,
                amount: vec![coin(500_000, JUNO)],
            };
            Ok(Response::new()
                .add_message(share)
                .add_attribute("action", "edit_post")
                .add_attribute("post_id", new_post.post_id.to_string())
                .add_attribute("editor", new_post.editor.unwrap()))
        }
    }
}
fn execute_delete_post(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    post_id: u64,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(Coin::new(10_000_000, JUNO)))?;
    POST.remove(deps.storage, post_id);
    let counter = ARTICLE_COUNT.load(deps.storage)?;
    let updated_counter = counter - 1;
    ARTICLE_COUNT.save(deps.storage, &updated_counter)?;
    Ok(Response::new()
        .add_attribute("action", "delete_post")
        .add_attribute("post_id", post_id.to_string()))
}
fn execute_like_post(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    post_id: u64,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(coin(10_000, JUNO)))?;
    let post = POST.load(deps.storage, post_id)?;
    let liked_post: Post = Post {
        post_id: post.post_id,
        post_title: post.post_title,
        external_id: post.external_id,
        text: post.text,
        tags: post.tags,
        author: post.author,
        creation_date: post.creation_date,
        last_edit_date: post.last_edit_date,
        editor: post.editor,
        likes: post.likes + 1,
    };
    POST.save(deps.storage, post_id, &liked_post)?;
    Ok(Response::new()
        .add_attribute("action", "like post")
        .add_attribute("post_id", post_id.to_string()))
}
fn execute_withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    let balance = deps.querier.query_all_balances(&env.contract.address)?;
    let bank_msg = BankMsg::Send {
        to_address: ADDRESS.to_string(),
        amount: balance,
    };

    let resp = Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw");
    Ok(resp)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPosts { limit, start_after } => query_all_posts(deps, env, limit, start_after),
        QueryMsg::Post { post_id } => query_post(deps, env, post_id),
        QueryMsg::ArticleCount {} => query_article_count(deps, env),
        QueryMsg::ProfileName { address } => query_profile_name(deps, env, address),
    }
}

//pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

fn query_all_posts(
    deps: Deps,
    _env: Env,
    limit: Option<u32>,
    start_after: Option<u64>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let posts = POST
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllPostsResponse { posts })
}

fn query_post(deps: Deps, _env: Env, post_id: u64) -> StdResult<Binary> {
    let post = POST.may_load(deps.storage, post_id)?;
    to_binary(&PostResponse { post })
}
fn query_article_count(deps: Deps, _env: Env) -> StdResult<Binary> {
    let article_count = ARTICLE_COUNT.load(deps.storage)?;
    to_binary(&ArticleCountResponse { article_count })
}
fn query_profile_name(deps: Deps, _env: Env, address: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address)?;
    let profile_name = PROFILE_NAME.may_load(deps.storage, validated_address)?;
    to_binary(&ProfileNameResponse { profile_name })
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = get_contract_version(deps.storage)?;
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    //canonical way from official docs
    #[allow(clippy::cmp_owned)]
    if ver.version > (*CONTRACT_VERSION).to_string() {
        return Err(StdError::generic_err("Must upgrade from a lower version").into());
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default()
        .add_attribute("action", "migration")
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("contract", CONTRACT_NAME))
}
