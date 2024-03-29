//cargo tarpaulin --ignore-tests = 97.56%
use crate::contract::{execute, instantiate, migrate, query};
use crate::msg::{
    AllPostsResponse, ArticleCountResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, PostResponse,
    ProfileNameResponse, QueryMsg,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, coin, from_binary, Response};

pub const ADDR1: &str = "juno1xh3mylsdmpvn0cp8mpz6uja34nev9w7ur8f945";
pub const ADDR2: &str = "addr2";

const JUNO: &str = "ujuno";

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
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
fn test_create_post_with_profile_name() {
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
            //URL too long
            external_id: "https://alxandri.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTnjvkdfkvdfvnksdnvkjdfnskvfndsnvjsdfkfdvkvfjnkfjknfvjkfdsvjdf".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: "Text".to_string(),
        };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    let msg = ExecuteMsg::CreatePost {
        post_title: "Different Title".to_string(),
        external_id: "https://alxandri.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTnjvkdfkvdfvnksdnvkjdfnskvfndsnvjsdfkfdvkvfjnkfjknfvjkfdsvjdf".to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        //too much text
        text: "nvdjsknjvkdfvksdfnjkvdfjksvnsdfjknvjksdfnjvsfnjkvdfnskvnsdfjknvjksdjkvjkdsfnvnsdfkvnjsdfnvjksdfnvnsdfvndfjsnvdlsfnvklsdfnvjkdfnvjfkfdnsjkvdfnsvnjkdsnvkdnskvnfkdsnvnjkfdnkvdfnsjvfnvjkfdsnvjkdfsnvjkdsnvdsfknvdfjknvsdvjdfnjklvnsdfjnvsdfknvjkdfnjkvdfnjksvnjdfnvkdfnvjkdfnvjkdfnvjkdfnvjkdfnvjkdfnvjknjdksvnjksdfnvjkdfnvjkdjskvnjkdsvsnfjdksnvksdflnsnvdjsknjvkdfvksdfnjkvdfjksvnsdfjknvjksdfnjvsfnjkvdfnskvnsdfjknvjksdjkvjkdsfnvnsdfkvnjsdfnvjksdfnvnsdfvndfjsnvdlsfnvklsdfnvjkdfnvjfkfdnsjkvdfnsvnjkdsnvkdnskvnfkdsnvnjkfdnkvdfnsjvfnvjkfdsnvjkdfsnvjkdsnvdsfknvdfjknvsdvjdfnjklvnsdfjnvsdfknvjkdfnjkvdfnjksvnjdfnvkdfnvjkdfnvjkdfnvjkdfnvjkdfnvjkdfnvjknjdksvnjksdfnvjkdfnvjkdjskvnjkdsvsnfjdksnvksdflns".to_string(),
    };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    let msg = ExecuteMsg::CreatePost {
        post_title: "Another Different Title".to_string(),
        //not alXandria gateway
        external_id: "https://alxandri.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7k".to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "nv".to_string(),
    };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_execute_create_post_invalid_duplicate_titles() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instatiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //new post same title
    let msg = ExecuteMsg::CreatePost {
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Duplicate".to_string(),
            "Post".to_string(),
            "Rejected".to_string(),
        ],
        text: "Hi".to_string(),
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
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    let info = mock_info(ADDR1, &[coin(1_900_000, JUNO)]);
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
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    let msg = ExecuteMsg::EditPost {
        post_id: 1,
        //URL too long
        external_id: "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTvnfjkvndfjknvdfnjkvvjndfjkvldnsjsdnvklsnnjksndjkvjkdfsnvjkdfnnsdjkvndfks".to_string(),
        text: "Text".to_string(),
        tags: vec!["Tax".to_string(), "Website".to_string()],
    };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    let msg = ExecuteMsg::EditPost {
        post_id: 1,
        //must use alXandria gateway
        external_id: "https://stake.tax/".to_string(),
        text: "Text".to_string(),
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
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    let info = mock_info(ADDR1, &[coin(10_000_000, JUNO)]);
    let msg = ExecuteMsg::DeletePost { post_id: 1 };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //query deleted post
    let msg = QueryMsg::Post { post_id: 1 };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PostResponse = from_binary(&bin).unwrap();
    assert!(res.post.is_none());
    //ensure same title is available
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
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
#[test]
fn test_execute_delete_post_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    let info = mock_info(ADDR1, &[coin(1, JUNO)]);
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
        start_after: Some(2),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: AllPostsResponse = from_binary(&bin).unwrap();
    //checks descending order
    assert_eq!(res.posts.len(), 1);
    let msg = QueryMsg::AllPosts {
        limit: None,
        start_after: None,
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: AllPostsResponse = from_binary(&bin).unwrap();
    println!("{:?}", res);
    //uncomment below line to verify order is descending
    // assert!(res.posts.is_empty());
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
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    //switch to is_none to intentionally fail and check output to verify profile name
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
    let _err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
    //attempt to register same profile name with different account
    let info = mock_info(ADDR2, &[]);
    let msg = ExecuteMsg::RegisterProfileName {
        profile_name: "v i T".to_string(),
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
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
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
    let info = mock_info(ADDR1, &[coin(10_000, JUNO)]);
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
#[test]
fn test_execute_admin_create_post_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instatiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, JUNO)]);
    //new execute message
    let msg = ExecuteMsg::AdminCreatePost {
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
        address: "juno1x23423".to_string(),
        creation: "1920382392".to_string(),
        edit_date: "1832983".to_string(),
        editor_address: "juno8243989".to_string(),
        like_number: 5,
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
#[test]
fn test_execute_admin_create_post_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instatiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    //have post creation be from non-admin account, failing
    let info = mock_info(ADDR2, &[coin(1_000_000, JUNO)]);
    //new execute message
    let msg = ExecuteMsg::AdminCreatePost {
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
        address: "juno1x23423".to_string(),
        creation: "1920382392".to_string(),
        edit_date: "1832983".to_string(),
        editor_address: "juno8243989".to_string(),
        like_number: 5,
    };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_admin_register_profile_name() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::AdminRegisterProfileName {
        address: info.sender.to_string(),
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
    //switch to is_none to intentionally fail and check output to verify profile name
    assert!(res.profile_name.is_some())
}
#[test]
fn test_admin_register_profile_name_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set address to non-admin account to fail
    let info = mock_info(ADDR2, &[]);
    //register profile
    let msg = ExecuteMsg::AdminRegisterProfileName {
        address: info.sender.to_string(),
        profile_name: "v i T".to_string(),
    };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
