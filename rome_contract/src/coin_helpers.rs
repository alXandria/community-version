use crate::error::ContractError;
use cosmwasm_std::Coin;

pub fn assert_sent_exact_coin(
    sent: &[Coin],
    required: Option<Vec<Coin>>,
) -> Result<(), ContractError> {
    if let Some(required_coins) = required {
        let mut required_amounts = Vec::new();
        for coin in required_coins {
            required_amounts.push((coin.denom.to_string(), coin.amount.u128()));
        }
        let mut received_amounts = Vec::new();
        for coin in sent {
            let mut found = false;
            for (denom, amount) in received_amounts.iter_mut() {
                if denom == &coin.denom.to_string() {
                    *amount += coin.amount.u128();
                    found = true;
                    break;
                }
            }
            if !found {
                received_amounts.push((coin.denom.to_string(), coin.amount.u128()));
            }
        }
        let mut sent_sufficient_funds = false;
        for (required_denom, required_amount) in required_amounts.iter() {
            let mut found = false;
            for (received_denom, received_amount) in received_amounts.iter() {
                if required_denom == received_denom && *required_amount <= *received_amount {
                    sent_sufficient_funds = true;
                    found = true;
                    break;
                }
            }
            if !found {
                break;
            }
        }
        if sent_sufficient_funds {
            return Ok(());
        } else {
            let mut received_amount_strings = vec![];
            for (denom, received_amount) in received_amounts.iter() {
                received_amount_strings.push(received_amount.to_string() + " " + denom);
            }
            let mut required_amount_strings = vec![];
            for (denom, required_amount) in required_amounts.iter() {
                required_amount_strings.push(required_amount.to_string() + " " + denom);
            }
            return Err(ContractError::NotEnoughFunds {
                needed: convert_vector_of_string_slices_to_string(required_amount_strings),
                received: convert_vector_of_string_slices_to_string(received_amount_strings),
            });
        }
    }
    Ok(())
}

fn convert_vector_of_string_slices_to_string(vector: Vec<String>) -> String {
    let mut string = String::new();
    for s in vector {
        string.push_str(&s);
        string.push_str(", ");
    }
    string
}
