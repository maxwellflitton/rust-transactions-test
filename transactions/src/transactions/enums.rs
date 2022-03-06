use std::cmp::PartialEq;


/// This enum is responsible for defining the types of transactions that can be made. 
/// 
/// # Attributes
/// * DEPOSIT: a credit to the client's asset account, meaning it should increase the available and total funds of the client account
/// * WITHDRAWAL: is a debit to the client's asset account, meaning it should decrease the available and total funds of the client account
/// * DISPUTE: a client's claim that a transaction was erroneous and should be reversed
/// * RESOLVE: a resolution to a dispute, releasing the associated held funds
/// * CHARGEBACK: the final state of a dispute and represents the client reversing a transaction
#[derive(Debug, PartialEq, Clone)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAWAL,
    DISPUTE,
    RESOLVE,
    CHARGEBACK
}

impl TransactionType {

    /// A consructor for the TransactionType. 
    /// 
    /// # Arguments 
    /// * selection (String): the selection for the enum to be created on
    pub fn new(selection: String) -> TransactionType {
        match selection.as_str() {
            "deposit" => {return TransactionType::DEPOSIT},
            "withdrawal" => {return TransactionType::WITHDRAWAL},
            "dispute" => {return TransactionType::DISPUTE},
            "resolve" => {return TransactionType::RESOLVE},
            "chargeback" => {return TransactionType::CHARGEBACK},
            _ => {panic!("selection not supported")},
        }
    }
}
