// models/transaction.rs

use anchor_lang::prelude::*;

pub enum TransactionType {
  Buy,
  Sell,
  Rebalance,
}

#[program]
mod transaction {
    use super::*;

    pub struct UserTransaction {
      pub user_id: u64,
      pub transactions: Vec<Transaction>,
      pub transaction_id: u64,
  }

    // Define the Transaction state.
    pub struct Transaction {
        pub transaction_type: TransactionType,
        pub amount: f64,
    }

    // Initialize a Transaction.
    pub fn initialize(ctx: Context<InitializeTransaction>, user_id: u64) -> ProgramResult {
        let transaction = &mut ctx.accounts.transaction;
        transaction.user_id = user_id;
        transaction.transactions = Vec::new();
        transaction.transaction_id = 0;
       
        Ok(())
    }

    // add new transaction
}
