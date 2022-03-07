# rust-transactions-test
This repo houses a simple transaction engine. 

## Assumptions 
It's assumed that when the account has been locked 
no more transactions go through the account and there 
is no further effect on the account. 

## Running the application 
The application can be run with the following command:
```commandline
cargo run -- transactions.csv > outcome.csv
```
Where ```transactions.csv``` is the file that is 
being loaded, and the ```outcome.csv``` is where 
the result state of the accounts after processing the 
transactions. 
