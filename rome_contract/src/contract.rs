use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;
use random_number::random;

use crate::error::ContractError;
use crate::msg::{AllPostsResponse, ExecuteMsg, InstantiateMsg, PostResponse, QueryMsg};
use crate::state::{Config, Post, CONFIG, POST};

const CONTRACT_NAME: &str = "crates.io:alxandria";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or_else(|| info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePost {
            post_id,
            external_id,
            text,
            tags,
            author,
            creation_date
        } => execute_create_post(
            deps, 
            env, 
            info, 
            post_id, 
            external_id, 
            text, 
            tags, 
            author,
            creation_date
        ),
        ExecuteMsg::EditPost {
            post_id,
            external_id,
            text,
            tags,
            author,
            editor,
            creation_date,
            last_edit_date,
        } => execute_edit_post(
            deps,
            env,
            info,
            post_id,
            external_id,
            text,
            tags,
            author,
            editor,
            creation_date,
            last_edit_date,
        ),
        ExecuteMsg::DeletePost {
            post_id,
            external_id,
            text,
            tags,
            author,
            creation_date,
            last_edit_date,
            deleter,
            editor,
        } => execute_delete_post(
            deps,
            env,
            info,
            post_id,
            external_id,
            text,
            tags,
            author,
            creation_date,
            last_edit_date,
            deleter,
            editor,
        ),
    }
}

fn execute_create_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_id: u64,
    external_id: String,
    text: Option<String>,
    tags: Vec<String>,
    _author: String,
    _creation_date: String,
) -> Result<Response, ContractError> {
    if text.is_some() {
        return Err(ContractError::NoTextAllowed {});
    }
    let author = info.sender.to_string();
    let validated_author = deps.api.addr_validate(&author)?;
    let post: Post = Post {
        post_id: random!(),
        external_id,
        text,
        tags,
        author: validated_author.to_string(),
        creation_date: env.block.time.to_string(),
        last_edit_date: None,
        deleter: None,
        editor: None,
    };
    POST.save(deps.storage, post_id, &post)?;

    Ok(Response::new())
}

fn execute_edit_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_id: u64,
    external_id: String,
    text: Option<String>,
    tags: Vec<String>,
    _author: String,
    _editor: String,
    _creation_date: String,
    _last_edit_date: String,
) -> Result<Response, ContractError> {
    if text.is_some() {
        return Err(ContractError::NoTextAllowed {});
    }
    let post = POST.load(deps.storage, post_id)?;
    let editor = info.sender.to_string();
    let validated_editor = deps.api.addr_validate(&editor)?;
    let new_post: Post = Post {
        post_id: post.post_id,
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
    Ok(Response::new())
}
fn execute_delete_post(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    post_id: u64,
    external_id: String,
    text: Option<String>,
    tags: Vec<String>,
    _author: String,
    _creation_date: String,
    last_edit_date: Option<String>,
    _deleter: Option<String>,
    _editor: Option<String>,
) -> Result<Response, ContractError> {
    if text.is_some() || !external_id.is_empty() || !tags.is_empty() {
        return Err(ContractError::DeletedPost {});
    }
    let post = POST.load(deps.storage, post_id)?;
    let deleter = info.sender.to_string();
    let validated_deleter = deps.api.addr_validate(&deleter)?;
    let deleted_post: Post = Post {
        post_id: post.post_id,
        external_id,
        text,
        tags,
        author: post.author,
        creation_date: post.creation_date,
        last_edit_date,
        deleter: Some(validated_deleter.to_string()),
        editor: post.editor,
    };
    POST.save(deps.storage, post_id, &deleted_post)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPosts {} => query_all_posts(deps, env),
        QueryMsg::Post { post_id } => query_post(deps, env, post_id),
    }
}

fn query_all_posts(deps: Deps, _env: Env) -> StdResult<Binary> {
    let posts = POST
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllPostsResponse { posts })
}

fn query_post(deps: Deps, _env: Env, post_id: u64) -> StdResult<Binary> {
    let post = POST.may_load(deps.storage, post_id)?;
    to_binary(&PostResponse { post })
}

#[cfg(test)]
mod tests {
    use crate::contract::instantiate;
    use crate::msg::{AllPostsResponse, ExecuteMsg, InstantiateMsg, PostResponse, QueryMsg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary};
    use random_number::random;

    use super::{execute, query};

    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);

        let msg = InstantiateMsg { admin: None };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        )
    }
    #[test]
    fn test_instantiate_with_admin() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);

        let msg = InstantiateMsg {
            admin: Some(ADDR2.to_string()),
        };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR2)]
        )
    }
    #[test]
    fn test_execute_create_post_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        //instatiate
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        //new execute message
        let msg = ExecuteMsg::CreatePost {
            post_id: random!(),
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: None,
            author: info.sender.to_string(),
            creation_date: env.block.time.to_string()
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }
    #[test]
    fn test_execute_create_post_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        //new execute message
        let msg = ExecuteMsg::CreatePost {
            post_id: random!(),
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: Some("This will fail".to_string()),
            author: info.sender.to_string(),
            creation_date: env.block.time.to_string()
        };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
    #[test]
    fn test_execute_edit_post_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        //create a post
        let msg = ExecuteMsg::CreatePost {
            post_id: 16409,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: None,
            author: info.sender.to_string(),
            creation_date: env.block.time.to_string()
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        //edit message
        let msg = ExecuteMsg::EditPost {
            post_id: 16409,
            external_id: "https://stake.tax/".to_string(),
            text: None,
            tags: vec!["Tax".to_string(), "Website".to_string()],
            author: "desmos1d2wmr92lphgtpv9xl9ux2cssd5ras7t8atryzy".to_string(),
            editor: info.sender.to_string(),
            creation_date: "20220921212209".to_string(),
            last_edit_date: env.block.time.to_string(),
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }
    #[test]
    fn test_execute_edit_post_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        //edit a post and add text (fail)
        let msg = ExecuteMsg::CreatePost {
            post_id: 16409,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: None,
            author: info.sender.to_string(),
            creation_date: env.block.time.to_string()
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::EditPost {
            post_id: 16409,
            external_id: "https://stake.tax/".to_string(),
            text: Some("This will fail".to_string()),
            tags: vec!["Tax".to_string(), "Website".to_string()],
            author: "desmos1d2wmr92lphgtpv9xl9ux2cssd5ras7t8atryzy".to_string(),
            editor: info.sender.to_string(),
            creation_date: "20220921212209".to_string(),
            last_edit_date: env.block.time.to_string(),
        };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
    #[test]
    fn test_execute_delete_post_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        //create a post
        let msg = ExecuteMsg::CreatePost {
            post_id: 16409,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: None,
            author: info.sender.to_string(),
            creation_date: env.block.time.to_string()
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        //delete message
        let msg = ExecuteMsg::DeletePost {
            post_id: 16409,
            external_id: "".to_string(),
            text: None,
            tags: vec![],
            author: "desmos1d2wmr92lphgtpv9xl9ux2cssd5ras7t8atryzy".to_string(),
            creation_date: "20220921212209".to_string(),
            last_edit_date: Some(env.block.time.to_string()),
            deleter: Some(info.sender.to_string()),
            editor: None,
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }
    #[test]
    fn test_execute_delete_post_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreatePost {
            post_id: 16409,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: None,
            author: info.sender.to_string(),
            creation_date: env.block.time.to_string()
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::DeletePost {
            post_id: 16409,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            text: None,
            tags: vec!["".to_string()],
            author: "desmos1d2wmr92lphgtpv9xl9ux2cssd5ras7t8atryzy".to_string(),
            creation_date: "20220921212209".to_string(),
            last_edit_date: Some(env.block.time.to_string()),
            deleter: Some(info.sender.to_string()),
            editor: None,
        };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
    #[test]
    fn test_query_all_posts() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreatePost {
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: None,
            author: info.sender.to_string(),
            creation_date: env.block.time.to_string()
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreatePost {
            post_id: 2,
            external_id: "https://www.google.com".to_string(),
            tags: vec!["Search".to_string(), "Google".to_string()],
            text: None,
            author: info.sender.to_string(),
            creation_date: env.block.time.to_string()
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let msg = QueryMsg::AllPosts {};
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: AllPostsResponse = from_binary(&bin).unwrap();
        assert_eq!(res.posts.len(), 2);
    }
    #[test]
    fn test_query_post() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreatePost {
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: None,
            author: info.sender.to_string(),
            creation_date: env.block.time.to_string()
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        //query post
        let msg = QueryMsg::Post { post_id: 1 };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: PostResponse = from_binary(&bin).unwrap();
        assert!(res.post.is_some());
        //query nonexistent post
        let msg = QueryMsg::Post { post_id: 78476 };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: PostResponse = from_binary(&bin).unwrap();
        assert!(res.post.is_none());
    }
}
