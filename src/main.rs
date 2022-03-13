use std::io;
use std::env;

use csv;

mod data_access_layer;
mod transactions;
mod accounts;

use accounts::log_transaction;
use accounts::account_map::AccountMap;
use data_access_layer::schema::{TransactionSchema, AccountSchema};


fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[args.len() - 1];

    let mut reader = csv::Reader::from_path(file_path).unwrap();
    let mut account_map = AccountMap::new();

    for result in reader.deserialize() {
        let raw_transaction: TransactionSchema = result.unwrap();
        let transaction = raw_transaction.convert_to_transaction();
        account_map = log_transaction(Some(account_map), transaction);
    }

    let buffer = account_map.accounts.into_iter().map(|x|{AccountSchema::convert_from_account(x.1)}).collect::<Vec<AccountSchema>>();
    
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for account in buffer {
        wtr.serialize(account);
    }
}
