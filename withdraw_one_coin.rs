fn execute_withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    //verify wallet address is hardcoded admin
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    //go through balances owned by contract and send to ADMIN
    let balance = deps.querier.query_all_balances(&env.contract.address)?;
    let bank_msg = BankMsg::Send {
        to_address: ADDRESS.to_string(),
        amount: balance,
    };
    let resp = Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw");
    Ok(resp)
}