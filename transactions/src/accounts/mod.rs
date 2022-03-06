pub mod account;
pub mod account_map;

use account_map::AccountMap;
use super::transactions::transaction::Transaction;


/// Entrypoint for logging a transaction to an account. 
/// 
/// # Arguments 
/// * current_state (Option<AccountMap>): the map of all the acounts and transactions with those accounts (if None a new one is created)
/// * transaction_type (Transaction): the transaction to be logged 
/// 
/// # Returns 
/// * (AccountMap): the updated map of all the accounts and transactions
pub fn log_transaction(current_state: Option<AccountMap>, transaction: Transaction) -> AccountMap {

    let mut account_state: AccountMap;

    match current_state {
        Some(account_data) => {
            account_state = account_data;
        }, 
        None => {
            account_state = AccountMap::new();
        }
    }

    account_state = account_state.add_transaction(transaction.clone(), transaction.client);

    return account_state
}