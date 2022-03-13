use super::enums::TransactionType;


/// This struct is responsible for housing data for inividual transactions.
/// 
/// # Attributes 
/// * transaction_type (TransactionType): the type of transaction being made 
/// * client (i32): the ID of the account making the transaction 
/// * tx (i32): the ID of the transaction 
/// * amount (Option<f32>): the amount involved for the transaction
#[derive(Debug, Clone)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: i32,
    pub tx: i32,
    pub amount: Option<f32>
}
