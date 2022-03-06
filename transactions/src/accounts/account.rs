use core::panic;

use crate::transactions::enums::TransactionType;

use super::super::transactions::transaction::Transaction; 
use super::super::transactions::enums::TransactionType::{CHARGEBACK, DEPOSIT, WITHDRAWAL, DISPUTE, RESOLVE};


/// This struct is responsible for housing data around an account and its transactions. 
/// 
/// # Attributes 
/// * id (i32): the ID of the account 
/// * amount_available (f32): the amount of funds available in the account 
/// * amount_held (f32): the amount of funds held for dispute
/// * total (f32): amount_available + amount_held
/// * locked (bool): if the account is locked then transactions cannot occur 
/// * transaction_log (Vec<Transaction>): transactions performed on the account
#[derive(Debug, Clone)]
pub struct Account {
    pub id: i32,
    pub amount_available: f32,
    pub amount_held: f32,
    pub total: f32,
    pub locked: bool,
    pub transaction_log: Vec<Transaction>
}

impl Account {

    /// The constructor for the Account struct. 
    /// 
    /// # Arguments 
    /// * id (i32): the ID for the account also known as client for the transaction 
    /// 
    /// # Returns 
    /// * (Account): the newly constructed account
    pub fn new(id: i32) -> Account {
        let transaction_log: Vec<Transaction> = Vec::new();
        return Account{
            id, 
            transaction_log, 
            amount_available: 0.0, 
            amount_held: 0.0,
            total: 0.0,
            locked: false
        }
    }

    /// Extracts previous transactions from the log based on the transaction ID and the type of transaction making the call. 
    /// 
    /// # Arguments 
    /// * transactions (&Vec<Transaction>): the transactions to be searched through for extraction
    /// * tx (&i32): the ID of the transaction being extracted 
    /// * transaction_type (&TransactionType): the type of transaction making the call 
    /// 
    /// # Returns 
    /// * (Option<&Transaction>) transaction under that ID and type needed if exists
    fn extract_transaction<'a>(transactions: &'a Vec<Transaction>, tx: &i32, transaction_type: &TransactionType) -> Option<&'a Transaction> {
        let mut extracted_transaction: Option<&Transaction> = None;
        let allowed_category: TransactionType;

        match transaction_type {
            RESOLVE => {
                allowed_category = DISPUTE;
            },
            CHARGEBACK => {
                allowed_category = DISPUTE;
            }
            DISPUTE => {
                allowed_category = DEPOSIT;
            },
            _ => {
                panic!("deposits and withdraws do not need to extract previous transactions");
            }
        }

        for logged_transaction in transactions {
            if &logged_transaction.tx == tx {

                if &allowed_category == &logged_transaction.transaction_type {
                    extracted_transaction = Some(logged_transaction);
                    break
                }
            }
        }
        return extracted_transaction
    }

    /// Adds a transaction to the account with different rules applying depending on the type of transaction. 
    /// 
    /// # Arguments 
    /// * transaction (Transaction): the transaction to be added to the account 
    /// 
    /// # Returns 
    /// * (Result<Self, &'static str>): a new updated account if successful, or an error if the rules for the transaction type has been breached
    pub fn add_transaction(mut self, transaction: Transaction) -> Result<Self, &'static str> {

        if transaction.client != self.id {
            panic!("transaction id: {} is not the same as account ID: {}", transaction.client, self.id);
        }

        if self.locked == true {
            return Err("account is locked")
        }
        let transaction_reference = &transaction.tx.clone(); // the reference is taken here if needed for disputes

        match transaction.transaction_type {
            CHARGEBACK => {
                // extract a dispute => return an error if not
                let dispute = Account::extract_transaction(&self.transaction_log, 
                                                                                      transaction_reference, 
                                                                                      &transaction.transaction_type);
                let disputed_transaction: &Transaction;
                match dispute {
                    None => {
                        return Err("no dispute found for the chargeback");
                    },
                    Some(dispute_transaction) => {
                        // directly unwrap because the dispute would not have been logged if the transaction being disputed didn't exist
                        disputed_transaction = Account::extract_transaction(&self.transaction_log, 
                                                                                                    transaction_reference, 
                                                                                                    &dispute_transaction.transaction_type).unwrap();
                        
                    }
                }
                // check the held funds are there => return an error if not 
                if self.amount_held < disputed_transaction.amount.unwrap() {
                    return Err("not enough held funds for the chargeback")
                }
                // decrease the funds by the amount
                self.amount_held -= disputed_transaction.amount.unwrap();
                self.total -= disputed_transaction.amount.unwrap();
                // freeze the acount
                self.locked = true;
            },
            DEPOSIT => {
                self.amount_available += transaction.amount.unwrap();
                self.total += transaction.amount.unwrap();
            },
            WITHDRAWAL => {
                if transaction.amount.unwrap() > self.amount_available {
                    return Err("not enough funds for withdrawal")
                }
                self.amount_available -= transaction.amount.unwrap();
                self.total -= transaction.amount.unwrap();
            },
            DISPUTE => {
                let disputed_transaction = Account::extract_transaction(&self.transaction_log, 
                                                                                                    transaction_reference, 
                                                                                                    &transaction.transaction_type);

                // process the effect of the dispute if the transaction was found
                match disputed_transaction {
                    Some(inner_transaction) => {
                           self.amount_available -= inner_transaction.amount.unwrap();
                           self.amount_held += inner_transaction.amount.unwrap();
                    },
                    None => {
                        // do nothing and return the state as it was before the dispute
                        return Ok(self)
                    }
                }
            },
            RESOLVE => {
                let logged_dispute = Account::extract_transaction(&self.transaction_log, 
                                                                                            transaction_reference, 
                                                                                            &transaction.transaction_type);

                match logged_dispute {
                    Some(inner_transaction) => {
                        let disputed_transaction = Account::extract_transaction(&self.transaction_log, 
                                                                                            transaction_reference, 
                                                                                            &inner_transaction.transaction_type).unwrap();
                        self.amount_available += disputed_transaction.amount.unwrap();
                        self.amount_held -= disputed_transaction.amount.unwrap();
                    }, 
                    None => {
                        return Ok(self)
                    }
                }
            }
        }
        self.transaction_log.push(transaction);

        return Ok(self)
    }

}


#[cfg(test)]
mod account_tests {

    use super::Account;
    use super::Transaction;
    use super::{CHARGEBACK, DEPOSIT, WITHDRAWAL, DISPUTE, RESOLVE};

    #[test]
    #[should_panic]
    fn test_wrong_client_transaction() {
        let tx_one =   Transaction{transaction_type: DEPOSIT,    client: 2, tx: 1, amount: Some(1.0)};
        let account_one = Account::new(1);
        let _ = account_one.add_transaction(tx_one);
    }

    #[test]
    fn test_transaction_log() {
        let tx_one =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 1, amount: Some(1.0)};
        let tx_two =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 2, amount: Some(1.0)};
        let mut account_one = Account::new(1);

        account_one = account_one.add_transaction(tx_one).unwrap();
        account_one = account_one.add_transaction(tx_two).unwrap();

        assert_eq!(2, account_one.transaction_log.len());
        assert_eq!(1, account_one.transaction_log[0].tx);
        assert_eq!(2, account_one.transaction_log[1].tx);
    }

    #[test]
    fn test_deposit() {
        let tx_one =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 1, amount: Some(4.0)};

        let mut account_one = Account::new(1);

        account_one = account_one.add_transaction(tx_one).unwrap();

        assert_eq!(4.0, account_one.amount_available);
        assert_eq!(4.0, account_one.total);
        println!("{:?}", account_one);
    }

    #[test]
    fn test_withdrawal() {
        let tx_one =   Transaction{transaction_type: WITHDRAWAL,    client: 1, tx: 1, amount: Some(2.5)};
        let mut account_one = Account::new(1);

        account_one.amount_available = 4.0;
        account_one.total = 4.0;

        account_one = account_one.add_transaction(tx_one).unwrap();
        assert_eq!(1.5, account_one.amount_available);
        assert_eq!(1.5, account_one.total);
    }

    #[test]
    #[should_panic]
    fn test_overwithdrawal() {
        let tx_one =   Transaction{transaction_type: WITHDRAWAL,    client: 1, tx: 1, amount: Some(20.0)};
        let mut account_one = Account::new(1);

        account_one.amount_available = 4.0;
        account_one.total = 4.0;
        account_one.add_transaction(tx_one).unwrap();
    }

    #[test]
    fn test_normal_dispute() {
        let tx_one =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 1, amount: Some(5.0)};
        let tx_two =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 2, amount: Some(10.0)};
        let tx_three =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 3, amount: Some(5.0)};

        let tx_four =   Transaction{transaction_type: DISPUTE,    client: 1, tx: 4, amount: None};
        let tx_five =   Transaction{transaction_type: DISPUTE,    client: 1, tx: 2, amount: None};

        let mut account_one = Account::new(1);

        account_one = account_one.add_transaction(tx_one).unwrap();
        account_one = account_one.add_transaction(tx_two).unwrap();
        account_one = account_one.add_transaction(tx_three).unwrap();

        account_one = account_one.add_transaction(tx_four).unwrap();

        assert_eq!(20.0, account_one.amount_available);
        assert_eq!(0.0, account_one.amount_held);
        assert_eq!(20.0, account_one.total);
        assert_eq!(false, account_one.locked);

        account_one = account_one.add_transaction(tx_five).unwrap();
        assert_eq!(10.0, account_one.amount_available);
        assert_eq!(10.0, account_one.amount_held);
        assert_eq!(20.0, account_one.total);
        assert_eq!(false, account_one.locked);
    }

    #[test]
    fn test_resolve() {
        let tx_one =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 1, amount: Some(5.0)};
        let tx_two =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 2, amount: Some(10.0)};
        let tx_three =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 3, amount: Some(5.0)};
        let tx_four =   Transaction{transaction_type: DISPUTE,    client: 1, tx: 2, amount: None};

        let mut account_one = Account::new(1);

        account_one = account_one.add_transaction(tx_one).unwrap();
        account_one = account_one.add_transaction(tx_two).unwrap();
        account_one = account_one.add_transaction(tx_three).unwrap();
        account_one = account_one.add_transaction(tx_four).unwrap();

        let tx_five =   Transaction{transaction_type: RESOLVE,    client: 1, tx: 3, amount: Some(5.0)};
        let tx_six =   Transaction{transaction_type: RESOLVE,    client: 1, tx: 2, amount: None};

        account_one = account_one.add_transaction(tx_five).unwrap();
        assert_eq!(10.0, account_one.amount_available);
        assert_eq!(10.0, account_one.amount_held);
        assert_eq!(20.0, account_one.total);
        assert_eq!(false, account_one.locked);

        account_one = account_one.add_transaction(tx_six).unwrap();
        assert_eq!(20.0, account_one.amount_available);
        assert_eq!(0.0, account_one.amount_held);
        assert_eq!(20.0, account_one.total);
        assert_eq!(false, account_one.locked);
    }

    #[test]
    fn test_chargeback() {
        let tx_one =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 1, amount: Some(5.0)};
        let tx_two =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 2, amount: Some(10.0)};
        let tx_three =   Transaction{transaction_type: DEPOSIT,    client: 1, tx: 3, amount: Some(5.0)};

        let tx_four =   Transaction{transaction_type: DISPUTE,    client: 1, tx: 2, amount: None};
        let tx_five =   Transaction{transaction_type: CHARGEBACK,    client: 1, tx: 2, amount: None};

        let mut account_one = Account::new(1);

        account_one = account_one.add_transaction(tx_one).unwrap();
        account_one = account_one.add_transaction(tx_two).unwrap();
        account_one = account_one.add_transaction(tx_three).unwrap();

        account_one = account_one.add_transaction(tx_four).unwrap();
        account_one = account_one.add_transaction(tx_five).unwrap();
        println!("{:?}", account_one);

        assert_eq!(10.0, account_one.amount_available);
        assert_eq!(0.0, account_one.amount_held);
        assert_eq!(10.0, account_one.total);
        assert_eq!(true, account_one.locked);
    }
}

