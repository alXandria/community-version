use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    coins, entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult,
};
use cw2::set_contract_version;

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
        } => execute_create_post(deps, env, info, post_id, external_id, text, tags),
        ExecuteMsg::EditPost {
            post_id,
            external_id,
            text,
            tags,
        } => execute_edit_post(deps, env, info, post_id, external_id, text, tags),
        ExecuteMsg::DeletePost { post_id } => execute_delete_post(deps, env, info, post_id),
    }
}

fn execute_create_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_id: u64,
    external_id: String,
    text: String,
    tags: Vec<String>,
) -> Result<Response, ContractError> {
    let fee = coins(100_000_000, "udesmos");
    if info.funds != fee {
        return Err(ContractError::NotEnoughFunds {});
    }
    if text.len() > 499 {
        return Err(ContractError::TooMuchText {});
    }
    let author = info.sender.to_string();
    let validated_author = deps.api.addr_validate(&author)?;
    let post: Post = Post {
        post_id,
        external_id,
        text,
        tags,
        author: validated_author.to_string(),
        creation_date: env.block.time.to_string(),
        last_edit_date: None,
        deleter: None,
        editor: None,
    };
    POST.save(deps.storage, post.post_id, &post)?;

    Ok(Response::new()
        .add_attribute("action", "Create Post")
        .add_attribute("Post ID", post_id.to_string())
        .add_attribute("Author", validated_author.to_string()))
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
    let fee = coins(200_000_000, "udesmos");
    if info.funds != fee {
        return Err(ContractError::NotEnoughFunds {});
    }
    if text.len() > 499 {
        return Err(ContractError::TooMuchText {});
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
    Ok(Response::new()
        .add_attribute("Action", "Edit Post")
        .add_attribute("Post ID", new_post.post_id.to_string())
        .add_attribute("Editor", new_post.editor.unwrap()))
}
fn execute_delete_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_id: u64,
) -> Result<Response, ContractError> {
    let fee = coins(1_000_000_000, "udesmos");
    if info.funds != fee {
        return Err(ContractError::NotEnoughFunds {});
    }
    let post = POST.load(deps.storage, post_id)?;
    let deleter = info.sender.to_string();
    let validated_deleter = deps.api.addr_validate(&deleter)?;
    let deleted_post: Post = Post {
        post_id: post.post_id,
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
    Ok(Response::new()
        .add_attribute("Action", "Delete Post")
        .add_attribute("Post ID", deleted_post.post_id.to_string())
        .add_attribute("Delete", deleted_post.deleter.unwrap()))
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
    use cosmwasm_std::{attr, coin, from_binary};

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
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let info = mock_info(ADDR1, &[coin(100_000_000, "udesmos")]);
        //new execute message
        let msg = ExecuteMsg::CreatePost {
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: "Hi".to_string(),
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
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            //text over 500 characters
            text: "This will fail vdfjkvjdfnksvkndsvjsndjkvnkjfnvnsdjkvnsdfnvjkdfnsvnjdksnvkldsnvjkdfnvjkfdnvkdnfjvkndjsknvjksdnknjfknvjkdsfnjvknskdnvjkndsjkvsjkdnvjksdfnvjksdfnvjkdfsnjvksvndfjkvnjsdkfnvjksdfnvkjlsdfvjnldsfknvjkdsvnjdksjkvcjkdnkm dkfs vkdnjkvndfkjsvjkfdnvjksdfnjkvkdfnvdnskvnsdfvjkdsnvjkdfnvjkdnvjksdnvjkdsvnjkdfnsdvfdknvjksdnvjfkdsnvjkdfsnvjksdnvjkfdsnvjkdsvlnsjknvjkdsnvjksdfnvkndsfjkvnjdskvnksdflvnjdknvjksdnvjkdfsnvjkdsnvjksdnvkdsnvfjkdnvjkdnvjkfndsvkdsfnjvksdnvsdfjklnvjdkslnvjdksnvjdfknvsdfjklnvdjksfnvjkdlsfnvkd".to_string(),
        };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
    #[test]
    fn test_execute_edit_post_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let info = mock_info(ADDR1, &[coin(100_000_000, "udesmos")]);
        //create a post
        let msg = ExecuteMsg::CreatePost {
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: "".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        //edit message
        let info = mock_info(ADDR1, &[coin(200_000_000, "udesmos")]);
        let msg = ExecuteMsg::EditPost {
            post_id: 1,
            external_id: "https://stake.tax/".to_string(),
            text: "".to_string(),
            tags: vec!["Tax".to_string(), "Website".to_string()],
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }
    #[test]
    fn test_execute_edit_post_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let info = mock_info(ADDR1, &[coin(100_000_000, "udesmos")]);
        //edit a post and add text (fail)
        let msg = ExecuteMsg::CreatePost {
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: "".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::EditPost {
            post_id: 1,
            external_id: "https://stake.tax/".to_string(),
            text: "This will fail vdfjkvjdfnksvkndsvjsndjkvnkjfnvnsdjkvnsdfnvjkdfnsvnjdksnvkldsnvjkdfnvjkfdnvkdnfjvkndjsknvjksdnknjfknvjkdsfnjvknskdnvjkndsjkvsjkdnvjksdfnvjksdfnvjkdfsnjvksvndfjkvnjsdkfnvjksdfnvkjlsdfvjnldsfknvjkdsvnjdksjkvcjkdnkm dkfs vkdnjkvndfkjsvjkfdnvjksdfnjkvkdfnvdnskvnsdfvjkdsnvjkdfnvjkdnvjksdnvjkdsvnjkdfnsdvfdknvjksdnvjfkdsnvjkdfsnvjksdnvjkfdsnvjkdsvlnsjknvjkdsnvjksdfnvkndsfjkvnjdskvnksdflvnjdknvjksdnvjkdfsnvjkdsnvjksdnvkdsnvfjkdnvjkdnvjkfndsvkdsfnjvksdnvsdfjklnvjdkslnvjdksnvjdfknvsdfjklnvdjksfnvjkdlsfnvkd".to_string(),
            tags: vec!["Tax".to_string(), "Website".to_string()],
        };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
    #[test]
    fn test_execute_delete_post_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let info = mock_info(ADDR1, &[coin(100_000_000, "udesmos")]);
        //create a post
        let msg = ExecuteMsg::CreatePost {
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: "".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        //delete message
        let info = mock_info(ADDR1, &[coin(1_000_000_000, "udesmos")]);
        let msg = ExecuteMsg::DeletePost { post_id: 1 };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }
    #[test]
    fn test_execute_delete_post_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let info = mock_info(ADDR1, &[coin(100_000_000, "udesmos")]);
        let msg = ExecuteMsg::CreatePost {
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: "".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::DeletePost { post_id: 1 };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
    #[test]
    fn test_query_all_posts() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let info = mock_info(ADDR1, &[coin(100_000_000, "udesmos")]);
        let msg = ExecuteMsg::CreatePost {
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: "".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreatePost {
            post_id: 2,
            external_id: "https://www.google.com".to_string(),
            tags: vec!["Search".to_string(), "Google".to_string()],
            text: "".to_string(),
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
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let info = mock_info(ADDR1, &[coin(100_000_000, "udesmos")]);
        let msg = ExecuteMsg::CreatePost {
            post_id: 1,
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: "".to_string(),
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
