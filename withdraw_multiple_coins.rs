fn execute_withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // Verify wallet address is hardcoded admin
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    // Get the balances owned by the contract
    let balance = deps.querier.query_all_balances(&env.contract.address)?;
    let mut response = Response::new().add_attribute("action", "withdraw");  // Create a Response object with the "action" attribute set to "withdraw"
    // Iterate through the balance coins
    for coin in balance {
        let to_address = match coin.denom {  // Check the denom of the coin
            "ujunox" => ADMIN_JUNO_ADDRESS,  // Set the to_address to the admin Juno address if the denom is "ujunox"
            "uatom" => ADMIN_ATOM_ADDRESS,  // Set the to_address to the admin Atom address if the denom is "uatom"
            _ => return Err(ContractError::InvalidDenom { denom: coin.denom.to_string() }),  // Return an error if the denom is not recognized
        };
        let bank_msg = BankMsg::Send {
            to_address: to_address.to_string(),  // Set the to_address of the message to the desired address
            amount: vec![coin],  // Set the amount of the message to the current coin
        };
        response = response.add_message(bank_msg);  // Add the message to the response
    }
}