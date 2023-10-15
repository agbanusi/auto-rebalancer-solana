
use anchor_lang::prelude::*;
use crate::models::{Transaction, TransactionType, UserTransaction};
use crate::models::User;
use crate::models::TotalPortfolio;
use crate::models::CryptoPortfolio;

#[program]
mod handle_transaction {
    use super::*;

    pub fn handle(ctx: Context<HandleTransaction>, transaction: Transaction) -> ProgramResult {
        match transaction.transaction_type {
            TransactionType::Buy => handle_buy(ctx, transaction.amount),
            TransactionType::Sell => handle_sell(ctx, transaction.amount),
            TransactionType::Rebalance => handle_rebalance(ctx),
        }
    }

    fn handle_buy(ctx: Context<HandleTransaction>, amount: f64, token: String) -> ProgramResult {
        let total_portfolio = &mut ctx.accounts.total_portfolio;
        let crypto_portfolio = &mut ctx.accounts.crypto_portfolio;
        let user = &mut ctx.accounts.user;

        // Validate that the user has sufficient funds for the purchase
        let crypto_value = total_portfolio.crypto_value;
        if amount > user.total_value {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Implement logic for handling a "Buy" transaction
        // Update portfolios, user total value, and create a transaction record
        crypto_portfolio.add_token(&token, amount);
        total_portfolio.update_crypto_value(amount);
        user.update_total_value(-amount); // Deduct the purchase amount from the user's total value

        ctx.accounts.transaction.add_transaction(TransactionType::Buy, amount);

        Ok(())
    }

    fn handle_sell(ctx: Context<HandleTransaction>, amount: f64, token: String) -> ProgramResult {
        let total_portfolio = &mut ctx.accounts.total_portfolio;
        let crypto_portfolio = &mut ctx.accounts.crypto_portfolio;
        let user = &mut ctx.accounts.user;

        // Validate that the user has sufficient assets for the sale
        if crypto_portfolio.tokens.is_empty() || crypto_portfolio.tokens[0].amount < amount {
            return Err(ErrorCode::InsufficientAssets.into());
        }

        if !crypto_portfolio.tokens.iter().any(|t| t.symbol == token) || amount > user.total_value {
            return Err(ErrorCode::InvalidTransaction.into());
        }

        // Implement logic for handling a "Sell" transaction
        // Update portfolios, user total value, and create a transaction record
        crypto_portfolio.remove_token(&token);
        total_portfolio.update_crypto_value(-amount);
        user.update_total_value(amount); // Add the sale amount to the user's total value

        ctx.accounts.transaction.add_transaction(TransactionType::Sell, amount);

        Ok(())
    }

    fn handle_rebalance_total_portfolio(ctx: Context<HandleTransaction>) -> ProgramResult {
        let total_portfolio = &mut ctx.accounts.total_portfolio;
        let crypto_portfolio = &mut ctx.accounts.crypto_portfolio;
    
        // Retrieve the target asset allocation percentages from the user's total portfolio
        let crypto_percentage = total_portfolio.crypto_allocation_percentage();
        let bonds_percentage = total_portfolio.bonds_allocation_percentage();
        let stocks_percentage = total_portfolio.stocks_allocation_percentage();
    
        // Calculate the necessary adjustments to align with the target allocation
        let crypto_adjustment = crypto_percentage - total_portfolio.crypto_percentage();
        let bonds_adjustment = bonds_percentage - total_portfolio.bonds_percentage();
        let stocks_adjustment = stocks_percentage - total_portfolio.stocks_percentage();
    
        // Calculate the total value of the portfolio
        let total_value = total_portfolio.total_value();
    
        // Implement logic to buy/sell assets to rebalance the portfolio
        // For simplicity, we'll assume equal rebalancing for all assets
        let crypto_value_to_buy_sell = (crypto_adjustment / 100.0) * total_value;
        let bonds_value_to_buy_sell = (bonds_adjustment / 100.0) * total_value;
        let stocks_value_to_buy_sell = (stocks_adjustment / 100.0) * total_value;
    
        // Ensure the user has sufficient funds for the rebalancing
        if crypto_value_to_buy_sell > ctx.accounts.user.total_value {
            return Err(ErrorCode::InsufficientFunds.into());
        }
    
        // Implement logic to buy/sell assets to rebalance the portfolio
        // For simplicity, we'll distribute the rebalancing equally across all crypto tokens
        let crypto_tokens = &mut crypto_portfolio.tokens;
        let num_tokens = crypto_tokens.len();
    
        if num_tokens == 0 {
            // No tokens to rebalance, return
            return Ok(());
        }
    
        for token in crypto_tokens.iter_mut() {
            // Calculate the adjustment for this token
            let target_amount = (crypto_adjustment / 100.0) * token.amount;
            let adjustment = target_amount - token.amount;
    
            // Update the token amount and the state of the token on the Solana blockchain
            token.amount = target_amount;
            //TODO: add sell or buy swap function from radium
            // Update the state of the token in the Solana blockchain
            crypto_portfolio.update_token(&token, adjustment);
        }
    
        // Update portfolios and create a transaction record
        total_portfolio.update_crypto_value(crypto_value_to_buy_sell);
        total_portfolio.update_bonds_value(bonds_value_to_buy_sell);
        total_portfolio.update_stocks_value(stocks_value_to_buy_sell);
        
        ctx.accounts.transaction.add_transaction(TransactionType::Rebalance, amount);
    
        Ok(())
    }

    fn handle_rebalance_crypto_portfolio(ctx: Context<HandleTransaction>) -> ProgramResult {
        let portfolio = &mut ctx.accounts.crypto_portfolio;
    
        // Calculate the total value of the crypto portfolio
        let total_crypto_value = portfolio.total_crypto_value();
    
        // Ensure there are tokens in the crypto portfolio
        let crypto_tokens = &mut portfolio.tokens;
        let num_tokens = crypto_tokens.len();
    
        if num_tokens == 0 {
            // No tokens to rebalance, return
            return Ok(());
        }
    
        // Iterate through each token in the crypto portfolio
        for token in crypto_tokens.iter_mut() {
            // Get the target allocation percentage for the token
            let target_percentage = token.allocation_percentage;
            let crypto_adjustment = target_percentage - portfolio.percentage(&token.symbol);
    
            // Calculate the target value for the token based on the target percentage    
            let target_amount = (crypto_adjustment / 100.0) * token.amount;
            let adjustment = target_amount - token.amount;
    
            // Update the token amount and the state of the token on the Solana blockchain
            token.amount = target_amount;
            //TODO: add sell or buy swap function from radium
            // Update the state of the token in the Solana blockchain
            portfolio.update_token(&token, adjustment);
        }

        ctx.accounts.transaction.add_transaction(TransactionType::Rebalance, amount);
    
        Ok(())
    }
    
}

#[error]
pub enum ErrorCode {
    #[msg("Insufficient funds for the transaction.")]
    InsufficientFunds,
    #[msg("Insufficient assets for the sale.")]
    InsufficientAssets,
}

