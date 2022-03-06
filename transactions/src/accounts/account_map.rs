use std::collections::HashMap;

use super::super::transactions::transaction::Transaction;
use super::account::Account;


/// This struct is responsible for managing the accounts that hold the transactions. 
/// 
/// # Attributes 
/// * accounts (HashMap<i32, Account>): holds the accounts that currently have transactions
/// * total_transaction_log (Vec<Transaction>): a log of all the successful transactions
/// * total_error_transaction_log (Vec<Transaction>): a log of all the unsuccessful transactions
pub struct AccountMap {
    pub accounts: HashMap<i32, Account>,
    pub total_transaction_log: Vec<Transaction>,
    pub total_error_transaction_log: Vec<Transaction>
}

impl AccountMap {

    /// The constructor for the AccountMap struct. 
    /// 
    /// # Returns
    /// * (AccountMap): constructed blank map for accounts
    pub fn new() -> AccountMap {
        let accounts: HashMap<i32, Account> = HashMap::new();
        let total_transaction_log: Vec<Transaction> = Vec::new();
        let total_error_transaction_log: Vec<Transaction> = Vec::new();
        return AccountMap{accounts, total_transaction_log, total_error_transaction_log}
    }

    /// Adds a transaction to an account creating a new account if it is not currently present. 
    /// 
    /// # Arguments 
    /// * transaction (Transaction): the transaction to be added
    /// * account_id (i32): the ID of the account to have the transaction added to 
    /// 
    /// # Returns 
    /// * (Self): the updated map with the new transaction and account if it was not present before
    pub fn add_transaction(mut self, transaction: Transaction, account_id: i32) -> Self {

        let mut account: Account;

        match self.accounts.get(&account_id) {
            Some(found_account) => {
                account = found_account.clone();
            },
            None => {
                account = Account::new(account_id);
            }
        }

        let transaction_result = account.add_transaction(transaction.clone());

        match transaction_result {
            Ok(new_state) => {
                account = new_state;
                self.accounts.insert(account_id, account);
                self.total_transaction_log.push(transaction);
            },
            Err(_) => {
                // println!("{}", message);
                self.total_error_transaction_log.push(transaction);
            }
        }
        return self
    }
}
