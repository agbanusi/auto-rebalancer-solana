// // models/user.rs

// pub struct User {
//   pub user_id: u64,
//   pub total_value: f64,
// }

// impl User {
//   pub fn new(user_id: u64, total_value: f64) -> Self {
//       User { user_id, total_value }
//   }

//   pub fn update_total_value(&mut self, new_value: f64) {
//       // Implement logic to update the total value of the user's portfolio.
//   }
// }

use anchor_lang::prelude::*;

#[program]
mod user {
    use super::*;

    // Define the User state.
    pub struct User {
        pub user_id: u64,
        pub total_value: f64,
    }

    // Initialize a User.
    pub fn initialize(ctx: Context<InitializeUser>, user_id: u64) -> ProgramResult {
        let user = &mut ctx.accounts.user;
        user.user_id = user_id;
        user.total_value = 0.0;
        Ok(())
    }

    // Update the total value of the user's portfolio.
    pub fn update_total_value(ctx: Context<UpdateTotalValue>, new_value: f64) -> ProgramResult {
        let user = &mut ctx.accounts.user;
        user.total_value += new_value;
        Ok(())
    }
}
