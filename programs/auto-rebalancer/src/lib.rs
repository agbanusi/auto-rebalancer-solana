
// lib.rs

use anchor_lang::prelude::*;

declare_id!("YourProgramIDHere");

#[program]
mod portfolio_program {
    use super::*;
    use crate::models::crypto_portfolio::*;
    use crate::models::total_portfolio::*;
    use crate::models::transaction::*;
    use crate::models::user::*;
    use crate::processor::handle_transaction::*;

    pub fn initialize(ctx: Context<InitializePortfolio>, user_id: u64) -> ProgramResult {
        let user = &ctx.accounts.user;
        user.initialize(user_id)?;

        let crypto_portfolio = &mut ctx.accounts.crypto_portfolio;
        crypto_portfolio.initialize(user_id)?;

        let total_portfolio = &mut ctx.accounts.total_portfolio;
        total_portfolio.initialize(user_id)?;

        let transaction = &mut ctx.accounts.transaction;
        transaction.initialize(user_id)?;
        Ok(())
    }

    // Entry point for adding a crypto token to the portfolio.
    pub fn add_crypto_token(ctx: Context<AddToken>, symbol: String, amount: f64) -> ProgramResult {
        // You can call the appropriate processor function here.
        crypto_portfolio::add_token(ctx, symbol, amount)
        Ok(())
    }

    // Entry point for removing a crypto token from the portfolio.
    pub fn remove_crypto_token(ctx: Context<RemoveToken>, symbol: String) -> ProgramResult {
        // You can call the appropriate processor function here.
        crypto_portfolio::remove_token(ctx, symbol)
        Ok(())
    }

    // Entry point for updating a crypto token's amount in the portfolio.
    pub fn update_crypto_token(ctx: Context<UpdateToken>, symbol: String, new_amount: f64) -> ProgramResult {
        // You can call the appropriate processor function here.
        crypto_portfolio::update_token(ctx, symbol, new_amount)
        Ok(())
    }

    // Entry point for handling transactions.
    pub fn call_transaction(ctx: Context<HandleTransaction>, transaction_type: TransactionType, amount: f64) -> ProgramResult {
        // You can call the appropriate processor function here based on the transaction type.
        let transaction = Transaction { transaction_type, amount: amount };
        handle_transaction.handle(ctx, transaction);
        Ok(()) 
    }
    
    // add
}
