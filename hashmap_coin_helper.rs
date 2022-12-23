use crate::error::ContractError;
use cosmwasm_std::Coin;
use std::collections::HashMap;

pub fn assert_sent_exact_coin(sent: &[Coin], required: Option<Vec<Coin>>) -> Result<(), ContractError> {
    if let Some(required_coins) = required {
        let mut required_amounts = HashMap::new();  // Create a HashMap to store the required amounts for each denom
        for coin in required_coins {  // Iterate through the required coins
            required_amounts.insert(coin.denom.to_string(), coin.amount.u128());  // Insert the required amount for each denom into the HashMap
        }
        let mut received_amounts = HashMap::new();  // Create a HashMap to store the received amounts for each denom
        for coin in sent {  // Iterate through the sent coins
            let entry = received_amounts.entry(coin.denom.to_string()).or_insert(0);  // Get the entry for the coin's denom in the received_amounts HashMap, or insert a default value of 0 if it does not exist
            *entry += coin.amount.u128();  // Add the coin's amount to the entry
        }
        let mut sent_sufficient_funds = false;  // Set sent_sufficient_funds to false initially
        for (denom, required_amount) in required_amounts.iter() {  // Iterate through the required amounts for each denom
            let received_amount = received_amounts.get(denom).unwrap_or(&0);  // Get the received amount for the denom from the received_amounts HashMap, or a default value of 0 if it does not exist
            if *required_amount <= *received_amount {  // Check if the required amount is less than or equal to the received amount
                sent_sufficient_funds = true;  // Set sent_sufficient_funds to true if the required amount is less than or equal to the received amount
                break;  // Break out of the loop since we have found a denom with sufficient funds
            }
        }
        if sent_sufficient_funds {  // Check if sent_sufficient_funds is true
            return Ok(());
        } else {
            let mut received_amount_strings = vec![];  // Create a vector to store the received amounts as strings
            for (denom, received_amount) in received_amounts.iter() {  // Iterate through the received amounts for each denom
                received_amount_strings.push(received_amount.to_string() + " " + denom);  // Add the received amount for each denom to the vector as a string
            }
            return Err(ContractError::NotEnoughFunds {
                needed: "10 junox or 10 uatom".to_string(),  // TODO: Replace this with a string representation of the required amounts for each denom
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
