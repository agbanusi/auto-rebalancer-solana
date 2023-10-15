// models/crypto_portfolio.rs

// pub struct CryptoToken {
//   pub symbol: String,
//   pub amount: f64,
// }

// pub struct CryptoPortfolio {
//   pub user_id: u64,
//   pub tokens: Vec<CryptoToken>,
// }

// impl CryptoPortfolio {
//   pub fn new(user_id: u64) -> Self {
//       CryptoPortfolio {
//           user_id,
//           tokens: Vec::new(),
//       }
//   }

//   pub fn add_token(&mut self, symbol: String, amount: f64) {
//       // Implement logic to add a new crypto token to the portfolio.
//       // Check if the token already exists and update the amount.
//       // If it doesn't exist, add it to the list.
//   }

//   pub fn remove_token(&mut self, symbol: &str) {
//       // Implement logic to remove a crypto token from the portfolio.
//       // Remove the token if it exists in the list.
//   }

//   pub fn update_token(&mut self, symbol: &str, new_amount: f64) {
//       // Implement logic to update the amount of a crypto token in the portfolio.
//   }
// }


use anchor_lang::prelude::*;

#[program]
mod crypto_portfolio {
    use super::*;

    // Define the CryptoToken struct.
    pub struct CryptoToken {
        pub symbol: String,
        pub amount: f64,
        pub address: String,
        pub allocation_percentage: f64;
    }

    // Define the CryptoPortfolio state.
    pub struct CryptoPortfolio {
        pub user_id: u64,
        pub tokens: Vec<CryptoToken>,
    }

    // Initialize the CryptoPortfolio.
    pub fn initialize(ctx: Context<InitializeCryptoPortfolio>, user_id: u64) -> ProgramResult {
        let portfolio = &mut ctx.accounts.crypto_portfolio;
        portfolio.user_id = user_id;
        portfolio.tokens = Vec::new();
        Ok(())
    }

    // Add a new crypto token to the portfolio.
    pub fn add_token(ctx: Context<AddToken>, symbol: String, amount: f64, address: String) -> ProgramResult {
      let portfolio = &mut ctx.accounts.crypto_portfolio;
      let new_token = CryptoToken {
          symbol,
          amount,
          address,
          allocation_percentage: 0.0,
      };
      portfolio.tokens.push(new_token);

      // Recalculate allocation percentages
      let total_value = portfolio.tokens.iter().map(|token| token.amount).sum::<f64>();
      for token in portfolio.tokens.iter_mut() {
          token.allocation_percentage = (token.amount / total_value) * 100.0;
      }

      ctx.accounts.user.total_value += amount;
      Ok(())
    }

    // Remove a crypto token from the portfolio.
    pub fn remove_token(ctx: Context<RemoveToken>, symbol: String) -> ProgramResult {
        let portfolio = &mut ctx.accounts.crypto_portfolio;
        let removed_amount = if let Some(index) = portfolio
            .tokens
            .iter()
            .position(|token| token.symbol == symbol)
        {
            portfolio.tokens.remove(index).amount
        } else {
            return Err(ErrorCode::TokenNotFound.into());
        };
        
        // Recalculate allocation percentages
        if removed_amount > 0.0 {
            let total_value = portfolio.tokens.iter().map(|token| token.amount).sum::<f64>();
            for token in portfolio.tokens.iter_mut() {
                token.allocation_percentage = (token.amount / total_value) * 100.0;
            }
        }

        ctx.accounts.user.total_value -= removed_amount;
        Ok(())
    }

    // Update the amount of a crypto token in the portfolio.
    pub fn update_token(ctx: Context<UpdateToken>, symbol: String, new_amount: f64) -> ProgramResult {
        
        let portfolio = &mut ctx.accounts.crypto_portfolio;
        if let Some(token) = portfolio.tokens.iter_mut().find(|t| t.symbol == symbol) {
            // Calculate the difference in amounts
            let amount_difference = new_amount - token.amount;

            // Update the token's amount
            token.amount = new_amount;

            // Recalculate allocation percentages
            let total_value = portfolio.tokens.iter().map(|t| t.amount).sum::<f64>();
            for t in portfolio.tokens.iter_mut() {
                t.allocation_percentage = (t.amount / total_value) * 100.0;
            }

            // Update the user's total value
            ctx.accounts.user.total_value += amount_difference;
        }
        Ok(())
    }

    // Calculate the total value of the portfolio.
    pub fn total_value(ctx: Context<TotalValue>) -> ProgramResult {
        let portfolio = &ctx.accounts.portfolio;
        let total_value = portfolio.tokens.iter().map(|t| t.amount).sum::<f64>();
        Ok(total_value)
    }

    // Calculate the percentage of a specific token's allocation in the portfolio.
    pub fn percentage(ctx: Context<Percentage>, symbol: String) -> ProgramResult {
        let portfolio = &ctx.accounts.portfolio;
        if let Some(token) = portfolio.tokens.iter().find(|t| t.symbol == symbol) {
            Ok(token.allocation_percentage)
        } else {
            Err(ErrorCode::TokenNotFound.into())
        }
    }
}
