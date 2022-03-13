use serde::{Deserialize, Serialize};

use super::super::transactions::enums::TransactionType;
use super::super::transactions::transaction::Transaction;
use super::super::accounts::account::Account;


/// This struct is responsible for Deserialising transactions from the CSV file. 
/// 
/// # Attributes 
/// * transaction_type (String): the type of transaction (can be called "type" in the CSV)
/// * client (i32): the ID of the user who is making the transaction 
/// * tx (i32): the ID of the transaction 
/// * amount (Option<f32>): the amount of the transaction
#[derive(Debug, Deserialize)]
pub struct TransactionSchema {
    #[serde(alias = "type")]
    pub transaction_type: String,
    pub client: i32,
    pub tx: i32,
    pub amount: Option<f32>
}

impl TransactionSchema {

    /// Concerts the struct into a Transaction struct. 
    /// 
    /// # returns 
    /// * (Transaction): the transaction struct fit for processing
    pub fn convert_to_transaction(self) -> Transaction {
        let transaction_type = TransactionType::new(self.transaction_type);
        return Transaction{transaction_type, client: self.client, tx: self.tx, amount: self.amount}
    }
}


/// This struct is responsible for serialising account data to be written to a CSV file. 
/// 
/// # Attributes 
/// * client (i32): the ID of the cient and thus the account 
/// * available (f32): the amount of funds available 
/// * held (f32): the amount of funds held 
/// * total (f32): the total amount of funds 
/// * locked (bool): if the account is locked or not 
#[derive(Debug, Serialize)]
pub struct AccountSchema {
    pub client: i32,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool
}

impl AccountSchema {

    /// Gets data from account that has been processed to be ready to be written. 
    /// 
    /// # Arguments 
    /// * account (Account): the account to be serialised
    /// 
    /// # Returns 
    /// (AccountSchema): the schema to be written to CSV 
    pub fn convert_from_account(account: Account) -> AccountSchema {
        return AccountSchema{client: account.id, available: account.amount_available, held: account.amount_held, total: account.total, locked: account.locked}
    }

}
