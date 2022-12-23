let required_coins = vec![Coin::new(10, "ujunox"), Coin::new(10, "uatom")];  // Create a vector of required coins with the desired amounts and denoms
assert_sent_exact_coin(&info.funds, Some(required_coins))?;  // Call the assert_sent_exact_coin function with the required coins
