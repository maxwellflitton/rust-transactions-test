use serde::{Deserialize, Serialize};

use super::super::transactions::enums::TransactionType;
use super::super::transactions::transaction::Transaction;
use super::super::accounts::account::Account;


#[derive(Debug, Deserialize)]
pub struct TransactionSchema {
    #[serde(alias = "type")]
    pub transaction_type: String,
    pub client: i32,
    pub tx: i32,
    pub amount: Option<f32>
}

impl TransactionSchema {

    pub fn convert_to_transaction(self) -> Transaction {
        let transaction_type = TransactionType::new(self.transaction_type);
        return Transaction{transaction_type, client: self.client, tx: self.tx, amount: self.amount}
    }
}


#[derive(Debug, Serialize)]
pub struct AccountSchema {
    pub client: i32,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool
}

impl AccountSchema {

    pub fn convert_from_account(account: Account) -> AccountSchema {
        return AccountSchema{client: account.id, available: account.amount_available, held: account.amount_held, total: account.total, locked: account.locked}
    }

}
