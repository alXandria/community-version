use crate::contract::{execute, instantiate, migrate, query};
use crate::msg::{
    AllPostsResponse, ArticleCountResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, PostResponse,
    ProfileNameResponse, QueryMsg,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, coin, from_binary, Response};

pub const ADDR1: &str = "juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4";
pub const ADDR2: &str = "addr2";

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR1)]
    )
}

#[test]
fn test_instantiate_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR2, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _err = instantiate(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn migrate_works() {
    //instantiate
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    //migrate
    let msg = MigrateMsg {};
    let _res: Response = migrate(deps.as_mut(), mock_env(), msg).unwrap();
}

#[test]
fn test_execute_create_post_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instatiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    //new execute message
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
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
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //new execute message
    let msg = ExecuteMsg::CreatePost {
            post_title: "Mintscan Prop 320".to_string(),
            //wrong URL
            external_id: "https://alxandri.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT".to_string(),
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
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    //create a post
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //edit message
    let info = mock_info(ADDR1, &[coin(1_900_000, "ujunox")]);
    let msg = ExecuteMsg::EditPost {
        post_id: 1,
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
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
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
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
            //too much text
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
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    //create a post
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //delete message
    let info = mock_info(ADDR1, &[coin(10_000_000, "ujunox")]);
    let msg = ExecuteMsg::DeletePost { post_id: 1 };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //query deleted post
    let msg = QueryMsg::Post { post_id: 1 };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: PostResponse = from_binary(&bin).unwrap();
    assert!(res.post.is_none());
}
#[test]
fn test_execute_delete_post_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    let msg = ExecuteMsg::DeletePost { post_id: 3 };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_withdraw_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    let msg = ExecuteMsg::WithdrawJuno {};
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
#[test]
fn test_withdraw_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR2, &[]);
    let msg = ExecuteMsg::WithdrawJuno {};
    let _res = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_query_all_posts() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    let msg = ExecuteMsg::CreatePost {
        post_title: "Google.com".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec!["Search".to_string(), "Google".to_string()],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let msg = QueryMsg::AllPosts {
        limit: None,
        //pagination
        start_after: Some(1),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: AllPostsResponse = from_binary(&bin).unwrap();
    //checks pagination
    assert_eq!(res.posts.len(), 1);
}
#[test]
fn test_query_post() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
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
#[test]
fn test_query_article_count() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //query article count
    let msg = QueryMsg::ArticleCount {};
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let _res: ArticleCountResponse = from_binary(&bin).unwrap();
}
#[test]
fn test_register_profile_name() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::RegisterProfileName {
        profile_name: "v i T".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //query profile name
    let msg = QueryMsg::ProfileName {
        address: info.sender.to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: ProfileNameResponse = from_binary(&bin).unwrap();
    println!("{:?}", res);
    //switch to is_none to intentionally fail and check output to verify editable is true
    assert!(res.profile_name.is_some())
}
#[test]
fn test_reregister_profile_name() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile name
    let msg = ExecuteMsg::RegisterProfileName {
        profile_name: "v i T".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //attempt to register second profile name, should fail
    let msg = ExecuteMsg::RegisterProfileName {
        profile_name: "satoshi".to_string(),
    };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_like_post() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //like post
    let info = mock_info(ADDR1, &[coin(10_000, "ujunox")]);
    let msg = ExecuteMsg::LikePost { post_id: 1 };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //query post
    let msg = QueryMsg::Post { post_id: 1 };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: PostResponse = from_binary(&bin).unwrap();
    println!("{:?}", res);
    //switch to is_none to intentionally fail and check output to verify like = 1
    assert!(res.post.is_some());
}
