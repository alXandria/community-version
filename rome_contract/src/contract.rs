use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use cw_storage_plus::Bound;
use is_false::is_false;
use std::env;

use crate::coin_helpers::assert_sent_exact_coin;
use crate::error::ContractError;
use crate::msg::{
    AllPostsResponse, ArticleCountResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, PostResponse,
    ProfileNameResponse, QueryMsg,
};
use crate::state::{
    Config, Post, ARTICLE_COUNT, CONFIG, LAST_POST_ID, POST, POST_TITLES, PROFILE_NAME,
    REVERSE_LOOKUP,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
//Withdraw address
const ADDRESS: &str = "juno1kxxz03ejkv2le4xmp0yda0xg40la3wp3t7wnrx";
//Admin wallet
const ADMIN: &str = "juno1xh3mylsdmpvn0cp8mpz6uja34nev9w7ur8f945";
//limit ipfs link size to prevent link duplication
const MAX_ID_LENGTH: usize = 128;
//Block size is limited so make sure text input is less than 500 characters
const MAX_TEXT_LENGTH: usize = 499;
//alXandria dedicated gateway
const IPFS: &str = "https://alxandria.infura-ipfs.io/ipfs/";
const JUNO: &str = "ujuno";
const ALTER: &str = "ibc/8301f2e358bbcbf0e44dffca61889bf21b086b57ac39d48be3164e68e443ccef";

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
        ExecuteMsg::WithdrawJuno {} => execute_withdraw_juno(deps, env, info),
        ExecuteMsg::AdminRegisterProfileName {
            profile_name,
            address,
        } => execute_admin_register_profile_name(deps, env, info, profile_name, address),
        ExecuteMsg::AdminCreatePost {
            post_title,
            external_id,
            text,
            tags,
            address,
            creation,
            edit_date,
            editor_address,
            like_number,
        } => execute_admin_create_post(
            deps,
            env,
            info,
            post_title,
            external_id,
            text,
            tags,
            address,
            creation,
            edit_date,
            editor_address,
            like_number,
        ),
    }
}
fn execute_register_profile_name(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    profile_name: String,
) -> Result<Response, ContractError> {
    //check to see if wallet has already registered a name, fail if so
    let existing_name_check = PROFILE_NAME.may_load(deps.storage, info.sender.clone())?;
    if existing_name_check.is_some() {
        return Err(ContractError::ProfileNameImmutable {});
    }
    //trim, remove any spaces, and lowercase the input
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
    //In future, fees will be turned on for post creation, reference line below.
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
    #[allow(clippy::single_char_pattern)]
    let formatted_title = post_title.trim().to_lowercase().replace(" ", "");
    let title_checker = POST_TITLES.may_load(deps.storage, formatted_title.clone())?;
    if title_checker.is_some() {
        return Err(ContractError::PostAlreadyExists {});
    }
    //load article count from state and increment
    let counter = ARTICLE_COUNT.load(deps.storage)?;
    let updated_counter = counter + 1;
    //load last post id from state and increment
    let last_post_id = LAST_POST_ID.load(deps.storage)?;
    let incremented_id = last_post_id + 1;
    //check to see if address is matched to a profile name
    let load = PROFILE_NAME.may_load(deps.storage, info.sender.clone())?;
    match load {
        //if profile name exists, save profile name to author
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
            //save incremented id, post, and incremented article count
            LAST_POST_ID.save(deps.storage, &incremented_id)?;
            POST.save(deps.storage, post.post_id, &post)?;
            ARTICLE_COUNT.save(deps.storage, &updated_counter)?;
            POST_TITLES.save(deps.storage, formatted_title, &post.post_id)?;
            Ok(Response::new()
                .add_attribute("action", "create_post")
                .add_attribute("post_id", post.post_id.to_string())
                .add_attribute("author", load))
        }
        //if no profile name is registered, save wallet address as author
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
            //save incremented id, post, and incremented article count
            LAST_POST_ID.save(deps.storage, &incremented_id)?;
            POST.save(deps.storage, post.post_id, &post)?;
            ARTICLE_COUNT.save(deps.storage, &updated_counter)?;
            POST_TITLES.save(deps.storage, formatted_title, &post.post_id)?;
            Ok(Response::new()
                .add_attribute("action", "create_post")
                .add_attribute("post_id", post.post_id.to_string())
                .add_attribute("author", info.sender.to_string()))
        }
    }
}
//clippy defaults to max value of 7
#[allow(clippy::too_many_arguments)]
fn execute_admin_create_post(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    post_title: String,
    external_id: String,
    text: String,
    tags: Vec<String>,
    address: String,
    creation: String,
    edit_date: String,
    editor_address: String,
    like_number: u64,
) -> Result<Response, ContractError> {
    //check to see if admin
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    //load article count from state and increment
    let counter = ARTICLE_COUNT.load(deps.storage)?;
    let updated_counter = counter + 1;
    //load last post id from state and increment
    let last_post_id = LAST_POST_ID.load(deps.storage)?;
    let incremented_id = last_post_id + 1;
    //check to see if address is matched to a profile name
    let post: Post = Post {
        post_id: incremented_id,
        post_title,
        external_id,
        text,
        tags,
        author: address,
        creation_date: creation,
        last_edit_date: Some(edit_date),
        editor: Some(editor_address),
        likes: like_number,
    };
    //save incremented id, post, and incremented article count
    LAST_POST_ID.save(deps.storage, &incremented_id)?;
    POST.save(deps.storage, post.post_id, &post)?;
    ARTICLE_COUNT.save(deps.storage, &updated_counter)?;
    Ok(Response::new()
        .add_attribute("action", "create_post")
        .add_attribute("post_id", post.post_id.to_string())
        .add_attribute("author", info.sender.to_string()))
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
    //ensure .2 of crypto denom was sent
    assert_sent_exact_coin(&info.funds, Some(vec![Coin::new(200_000, JUNO), Coin::new(3_000_000, ALTER)]))?;
    if text.len() > MAX_TEXT_LENGTH {
        return Err(ContractError::TooMuchText {});
    }
    if external_id.len() > MAX_ID_LENGTH {
        return Err(ContractError::OnlyOneLink {});
    }
    if is_false(external_id.starts_with(IPFS)) {
        return Err(ContractError::MustUseAlxandriaGateway {});
    }
    //load post by ID passed
    let post = POST.load(deps.storage, post_id)?;
    //make sure editor is valid address
    let editor = info.sender.to_string();
    let validated_editor = deps.api.addr_validate(&editor)?;
    //update post content
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
    //save post
    POST.save(deps.storage, post_id, &new_post)?;
    Ok(Response::new()
        .add_attribute("action", "edit_post")
        .add_attribute("post_id", new_post.post_id.to_string())
        .add_attribute("editor", new_post.editor.unwrap()))
}
fn execute_delete_post(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    post_id: u64,
) -> Result<Response, ContractError> {
    //ensure 10 of crypto denom was sent & Create a vector of required coins with the desired amounts and denoms
    let required_coins = vec![Coin::new(10_000_000, JUNO), Coin::new(300_000_000, ALTER)];
    assert_sent_exact_coin(&info.funds, Some(required_coins))?;
    let deleted_post = POST.load(deps.storage, post_id)?;
    #[allow(clippy::single_char_pattern)]
    let formatted_title = deleted_post
        .post_title
        .trim()
        .to_lowercase()
        .replace(" ", "");
    POST_TITLES.remove(deps.storage, formatted_title);
    //remove post from state via post id
    POST.remove(deps.storage, post_id);
    //load counter and decrement
    let counter = ARTICLE_COUNT.load(deps.storage)?;
    let updated_counter = counter - 1;
    //save decremented counter
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
    //// Create a vector of required coins with the desired amounts and denoms
    let required_coins = vec![Coin::new(10_000, JUNO)];
    // Call the assert_sent_exact_coin function with the required coins
    assert_sent_exact_coin(&info.funds, Some(required_coins))?;
    //load post and increment like count
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
    //save post with incremented like count
    POST.save(deps.storage, post_id, &liked_post)?;
    Ok(Response::new()
        .add_attribute("action", "like post")
        .add_attribute("post_id", post_id.to_string()))
}
fn execute_withdraw_juno(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    //verify wallet address is hardcoded admin
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    //go through balances owned by contract and send to ADMIN
    let balance = deps.querier.query_balance(env.contract.address, JUNO)?;
    let bank_msg = BankMsg::Send {
        to_address: ADDRESS.to_string(),
        amount: vec![balance.clone()],
    };
    let resp = Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw")
        .add_attribute("amount withdrawn", balance.to_string());
    Ok(resp)
}
fn execute_admin_register_profile_name(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    profile_name: String,
    address: String,
) -> Result<Response, ContractError> {
    //check to see if admin
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    //trim, remove any spaces, and lowercase the input
    #[allow(clippy::single_char_pattern)]
    let formatted_profile_name = profile_name.trim().to_lowercase().replace(" ", "");
    let validated_address = deps.api.addr_validate(&address)?;
    //1) Check to see if there is the desired profile name is registered
    PROFILE_NAME.save(
        deps.storage,
        validated_address.clone(),
        &formatted_profile_name,
    )?;
    REVERSE_LOOKUP.save(
        deps.storage,
        formatted_profile_name.clone(),
        &validated_address,
    )?;
    Ok(Response::new()
        .add_attribute("action", "create profile name")
        .add_attribute("new profile name", formatted_profile_name))
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

//pagination fields
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
        .range(deps.storage, None, start, Order::Descending)
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
    //canonical way from official docs https://docs.cosmwasm.com/docs/1.0/smart-contracts/migration/#migrate-which-updates-the-version-only-if-newer
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
