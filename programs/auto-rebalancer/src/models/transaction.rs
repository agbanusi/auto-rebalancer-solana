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

    // Add a new transaction to the user's transactions.
    pub fn add_transaction(ctx: Context<AddTransaction>, transaction_type: TransactionType, amount: f64) -> ProgramResult {
        let transaction = &mut ctx.accounts.transaction;
        let new_transaction = Transaction {
            transaction_type,
            amount,
        };
        transaction.transactions.push(new_transaction);
        transaction.transaction_id += 1;
        Ok(())
    }

    // Retrieve a specific transaction by its ID.
    pub fn get_transaction(ctx: Context<GetTransaction>, transaction_id: u64) -> ProgramResult {
        let transaction = &ctx.accounts.transaction;
        if let Some(transaction) = transaction.transactions.get(transaction_id as usize) {
          Ok(transaction.clone())
        } else {
            return Err(ErrorCode::TransactionNotFound.into());
        }
    }

    // List all transactions for the user.
    pub fn list_transactions(ctx: Context<ListTransactions>) -> ProgramResult {
        let transaction = &ctx.accounts.transaction;
        Ok(transaction.transactions.clone())
    }
}

