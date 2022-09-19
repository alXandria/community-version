#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, Api};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use random_number::rand::rngs::mock;
    use random_number::random;
    use crate::contract::instantiate;
    use crate::msg::{InstantiateMsg, ExecuteMsg};
    use crate::state::{Post, POST};

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
    #[test]
    fn test_execute_edit_post_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        let msg = InstantiateMsg{admin:None};
        let _res = instantiate(
            deps.as_mut(), 
            env.clone(), 
            info.clone(), 
            msg).unwrap();
        //create a post
        let msg = ExecuteMsg::CreatePost { 
            post_id: 16409, 
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(), 
            tags: vec!["Blockchain".to_string(), "Governance".to_string(), "Rejected".to_string()], 
            text: None, 
            author: info.sender.to_string(), 
        };
        let _res = execute(
            deps.as_mut(),
            env.clone(), 
            info.clone(), 
            msg).unwrap();
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
        let _res = execute(
            deps.as_mut(), 
            env.clone(), 
            info.clone(), 
            msg).unwrap();
    }
    #[test]
    fn test_execute_edit_post_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        let msg = InstantiateMsg{admin:None};
        let _res = instantiate(
            deps.as_mut(), 
            env.clone(), 
            info.clone(), 
            msg).unwrap();
        //edit a post and add text (fail)
        let msg = ExecuteMsg::CreatePost { 
            post_id: 16409, 
            external_id: "https://www.mintscan.io/osmosis/proposals/320".to_string(), 
            tags: vec!["Blockchain".to_string(), "Governance".to_string(), "Rejected".to_string()], 
            text: None, 
            author: info.sender.to_string(), 
        };
        let _res = execute(
            deps.as_mut(),
            env.clone(), 
            info.clone(), 
            msg).unwrap();
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
        let _err = execute(
            deps.as_mut(),
            env.clone(), 
            info.clone(), 
            msg).unwrap_err();
    }
}