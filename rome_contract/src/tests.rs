#[cfg(test)]
mod tests {
    use crate::contract::{instantiate, execute, query};
    use crate::msg::{AllPostsResponse, ExecuteMsg, InstantiateMsg, PostResponse, QueryMsg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, coin, from_binary};

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
            vec![attr("Action", "Instantiate"), attr("Admin", ADDR1)]
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
            vec![attr("Action", "Instantiate"), attr("Admin", ADDR2)]
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
        let info = mock_info(ADDR1, &[]);
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
        let info = mock_info(ADDR1, &[coin(100_000_000, "udesmos")]);
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
        let msg = QueryMsg::AllPosts { limit: None };
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