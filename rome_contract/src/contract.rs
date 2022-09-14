use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::{Addr, entry_point};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64, BlockInfo};
use cw2::set_contract_version;
use desmos_bindings::posts::models::{Entities, RawPostAttachment, ReplySetting, PostReference};
// use random_number::random;

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
            text,
            tags,
            author, 
            editor
         } => execute_edit_post(
            deps,
            env,
            info,
            post_id,
            text,
            tags,
            author,
            editor
         ),
         ExecuteMsg::DeletePost { 
            post_id, 
            signer
         } => execute_delete_post(
            deps,
            env,
            info,
            post_id,
            signer
         ),
    }
}

fn execute_create_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_id: u64,
    external_id: Option<String>,
    text: Option<String>,
    tags: Vec<String>,
    author: Addr,
) -> Result<Response, ContractError> {
    if text.is_some() {
        return Err(ContractError::NoTextAllowed {  });
    }
    //id is out of scope, make a random number and wrap it in Uint64
    // let mut input_id: Uint64 = cosmwasm_std::Uint64::from(random!());
    let post: Post = Post {
        post_id,
        external_id,
        text,
        tags,
        author: info.sender,
        creation_date: env.block.time.to_string(),
        last_edit_date: None,
    };
    POST.save(deps.storage, (post_id), &post)?;
    
    Ok(Response::new())
}

fn execute_edit_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_id: u64,
    external_id: Option<String>,
    text: Option<String>,
    tags: Vec<String>,
    author: Addr,
    editor: Addr,
    creation_date: String,
    last_edit_date: String,
) -> Result<Response, ContractError> {
    //post_id here helps sensibly load post
    let post = POST.may_load(deps.storage, post_id.clone())?;
    
    let post: Post = Post {
        post_id,
        external_id,
        text,
        tags,
        author: info.sender,
        creation_date: env.block.time.to_string(),
        last_edit_date: None,
    };
    POST.save(deps.storage, post_id, &post)?;
}
fn execute_delete_post(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    post_id: u64,
    signer: Addr
) -> Result<Response, ContractError> {
    unimplemented!()
} 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::attr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::contract::instantiate;
    use crate::msg::InstantiateMsg;

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
}
