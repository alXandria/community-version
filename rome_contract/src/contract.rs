use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::{Addr, entry_point};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use random_number::random;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, Post, POST};


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
    let admin = msg.admin.unwrap_or(info.sender.to_string());
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
    match msg{
        ExecuteMsg::CreatePost { 
            post_id,
            external_id,
            text, 
            tags, 
            author, 
        } => execute_create_post(
            deps, 
            env, 
            info,
            post_id,
            external_id,
            text,
            tags,
            author,
         ),
         ExecuteMsg::EditPost { 
            post_id,
            external_id, 
            text,
            tags,
            author, 
            editor,
            creation_date,
            last_edit_date
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
            last_edit_date
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
            editor
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
    author: String,
) -> Result<Response, ContractError> {
    if text.is_some() {
        return Err(ContractError::NoTextAllowed {  });
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
    author: String,
    editor: String,
    creation_date: String,
    last_edit_date: String,
) -> Result<Response, ContractError> {
    let post = POST.load(deps.storage, post_id.clone())?;
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
    author: String,
    creation_date: String,
    last_edit_date: Option<String>,
    deleter: Option<String>,
    editor: Option<String>,
) -> Result<Response, ContractError> {
    let post = POST.load(deps.storage, post_id.clone())?;
    let deleter = info.sender.to_string();
    let validated_deleter = deps.api.addr_validate(&deleter)?;
    let deleted_post: Post = Post {
        post_id,
        external_id,
        text,
        tags,
        author,
        creation_date,
        last_edit_date,
        deleter: Some(validated_deleter.to_string()),
        editor,
    };
    POST.save(deps.storage, post_id, &deleted_post)?;
    Ok(Response::new())
} 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, Api};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use random_number::rand::rngs::mock;
    use random_number::random;
    use crate::contract::instantiate;
    use crate::msg::{InstantiateMsg, ExecuteMsg};

    use super::execute;

    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg {admin: None};
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
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg {admin: Some(ADDR2.to_string())};
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
        let info = mock_info(ADDR1, &vec![]);
        //instatiate
        let msg = InstantiateMsg{admin:None};
        let _res = instantiate(
            deps.as_mut(), 
            env.clone(), 
            info.clone(), 
            msg)
            .unwrap();
        //new execute message
        let msg = ExecuteMsg::CreatePost { 
            post_id: random!(), 
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(), 
            tags: vec!["Blockchain".to_string(), "Governance".to_string(), "Rejected".to_string()], 
            text: None, 
            author: info.sender.to_string(), 
        };
        let _res = execute(
            deps.as_mut(),
            env, 
            info, 
            msg).unwrap();
    }
    #[test]
    fn test_execute_create_post_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        let msg = InstantiateMsg{admin:None};
        let _res = instantiate(
            deps.as_mut(), 
            env.clone(), 
            info.clone(), 
            msg)
            .unwrap();
        //new execute message
        let msg = ExecuteMsg::CreatePost { 
            post_id: random!(), 
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(), 
            tags: vec!["Blockchain".to_string(), "Governance".to_string(), "Rejected".to_string()], 
            text: Some("This will fail".to_string()), 
            author: info.sender.to_string(), 
        };
        let _err = execute(deps.as_mut(), 
        env, 
        info, 
        msg).unwrap_err();
    }
}
