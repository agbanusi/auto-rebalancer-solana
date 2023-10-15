// // models/total_portfolio.rs

// pub struct TotalPortfolio {
//   pub user_id: u64,
//   pub crypto_value: f64,
//   pub bonds_value: f64,
//   pub stocks_value: f64,
// }

// impl TotalPortfolio {
//   pub fn new(user_id: u64) -> Self {
//       TotalPortfolio {
//           user_id,
//           crypto_value: 0.0,
//           bonds_value: 0.0,
//           stocks_value: 0.0,
//       }
//   }

//   pub fn update_crypto_value(&mut self, new_value: f64) {
//       // Implement logic to update the crypto value in the total portfolio.
//   }

//   pub fn update_bonds_value(&mut self, new_value: f64) {
//       // Implement logic to update the bonds value in the total portfolio.
//   }

//   pub fn update_stocks_value(&mut self, new_value: f64) {
//       // Implement logic to update the stocks value in the total portfolio.
//   }
// }


use anchor_lang::prelude::*;

#[program]
mod total_portfolio {
    use super::*;

    // Define the TotalPortfolio state.
    pub struct TotalPortfolio {
        pub user_id: u64,
        pub crypto_value: f64,
        pub bonds_value: f64,
        pub stocks_value: f64,
        pub total_value: f64,
        pub crypto_allocation_percentage: f64;
        pub bonds_allocation_percentage: f64;
        pub stocks_allocation_percentage: f64;
    }

    // Initialize the TotalPortfolio.
    pub fn initialize(ctx: Context<InitializeTotalPortfolio>, user_id: u64) -> ProgramResult {
        let portfolio = &mut ctx.accounts.total_portfolio;
        portfolio.user_id = user_id;
        portfolio.crypto_value = 0.0; // Set the initial crypto value to 0.0
        portfolio.bonds_value = 0.0; // Set the initial bonds value to 0.0
        portfolio.stocks_value = 0.0; // Set the initial stocks value to 0.0
        portfolio.total_value = 0.0; // Set the initial total value to 0.0
        portfolio.crypto_allocation_percentage = 100.0;
        portfolio.bonds_allocation_percentage = 0.0;
        portfolio.stocks_allocation_percentage = 0.0;
        Ok(())
    }

    // Update the crypto allc in the total portfolio.
    pub fn update_crypto_allocation(ctx: Context<UpdateCryptoAllocation>, new_value: f64) -> ProgramResult {
        let portfolio = &mut ctx.accounts.portfolio;
        portfolio.crypto_allocation_percentage = new_value;
        Ok(())
    }

    // Update the bond allc in the total portfolio.
    pub fn update_bonds_allocation(ctx: Context<UpdateStocksAllocation>, new_value: f64) -> ProgramResult {
        let portfolio = &mut ctx.accounts.portfolio;
        portfolio.bonds_allocation_percentage = new_value;
        Ok(())
    }

    // Update the crypto value in the total portfolio.
    pub fn update_stocks_allocation(ctx: Context<UpdateStocksAllocation>, new_value: f64) -> ProgramResult {
        let portfolio = &mut ctx.accounts.portfolio;
        portfolio.stocks_allocation_percentage = new_value;
        Ok(())
    }


    // Update the crypto value in the total portfolio.
    pub fn update_crypto_value(ctx: Context<UpdateCryptoValue>, new_value: f64) -> ProgramResult {
        let portfolio = &mut ctx.accounts.portfolio;
        portfolio.crypto_value += new_value;
        portfolio.total_value  = portfolio.crypto_value + portfolio.bonds_value + portfolio.stocks_value;
        Ok(())
    }

    // Update the bonds value in the total portfolio.
    pub fn update_bonds_value(ctx: Context<UpdateBondsValue>, new_value: f64) -> ProgramResult {
        let portfolio = &mut ctx.accounts.portfolio;
        portfolio.bonds_value += new_value;
        portfolio.total_value  = portfolio.crypto_value + portfolio.bonds_value + portfolio.stocks_value;
        Ok(())
    }

    // Update the stocks value in the total portfolio.
    pub fn update_stocks_value(ctx: Context<UpdateStocksValue>, new_value: f64) -> ProgramResult {
        let portfolio = &mut ctx.accounts.portfolio;
        portfolio.stocks_value += new_value;
        portfolio.total_value  = portfolio.crypto_value + portfolio.bonds_value + portfolio.stocks_value;
        Ok(())
    }

    pub fn crypto_percentage(&self) -> f64 {
        // Calculate the crypto allocation percentage
        if self.total_value == 0.0 {
            // Avoid division by zero if total_value is zero
            return 0.0;
        }
        (self.crypto_value / self.total_value) * 100.0
    }

    pub fn bonds_percentage(&self) -> f64 {
        // Calculate the bonds allocation percentage
        if self.total_value == 0.0 {
            // Avoid division by zero if total_value is zero
            return 0.0;
        }
        (self.bonds_value / self.total_value) * 100.0
    }

    pub fn stocks_percentage(&self) -> f64 {
        // Calculate the stocks allocation percentage
        if self.total_value == 0.0 {
            // Avoid division by zero if total_value is zero
            return 0.0;
        }
        (self.stocks_value / self.total_value) * 100.0
    }
}
