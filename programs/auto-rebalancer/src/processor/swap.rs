use anchor_lang::prelude::*;
use jupiter_cpi::Jupiter;

#[program]
pub mod jup_swap {
    use super::*;

    pub fn token_swap(ctx: Context<Swap>, token_a_amount: u64, token_b_amount: u64, user_token_a_account: AccountInfo<'info>, user_token_b_account: AccountInfo<'info>) -> Result<()> {
      // Create a Jupiter CPI client.
      let jupiter_cpi = Jupiter::new(ctx.accounts.jupiter_program.to_account_info());

      // Swap tokens.
      let swap_result = jupiter_cpi.swap(
          user_token_a_account.to_account_info(),
          user_token_b_account.to_account_info(),
          token_a_amount,
          token_b_amount,
      )?;

      // Handle the swap result.
      if swap_result.is_success() {
          // The swap was successful.
      } else {
        return Err(ErrorCode::SwapFailed.into());
      }

      Ok(())
  }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user_token_a_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_b_account: AccountInfo<'info>,
    pub jupiter_program: Program<'info, Jupiter>,
}

#[error]
pub enum ErrorCode {
    #[msg("Swap Failed!")]
    SwapFailed,
}
