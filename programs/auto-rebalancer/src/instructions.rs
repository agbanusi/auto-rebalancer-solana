use crate::processor::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeCryptoPortfolio<'info> {
    #[account(init, payer = user, space = 8 + 16 * tokens.len())]
    pub portfolio: ProgramAccount<'info, CryptoPortfolio>,
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddToken<'info> {
    #[account(mut)]
    pub portfolio: ProgramAccount<'info, CryptoPortfolio>,
}

#[derive(Accounts)]
pub struct RemoveToken<'info> {
    #[account(mut)]
    pub portfolio: ProgramAccount<'info, CryptoPortfolio>,
}

#[derive(Accounts)]
pub struct UpdateToken<'info> {
    #[account(mut)]
    pub portfolio: ProgramAccount<'info, CryptoPortfolio>,
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TotalValue<'info> {
    pub portfolio: ProgramAccount<'info, CryptoPortfolio>,
}

#[derive(Accounts)]
pub struct Percentage<'info> {
    pub portfolio: ProgramAccount<'info, CryptoPortfolio>,
}

impl<'info> InitializeCryptoPortfolio<'info> {
    pub fn accounts<'a, 'b: 'a>(&'a self, user: &'b AccountInfo<'info>) -> Result<CpiContext<'a, 'b>, ProgramError> {
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let cpi_program = self.portfolio.to_account_info().owner;
        let cpi_accounts = CpiAccount {
            host: user.clone(),
            owner: user.clone(),
            data: user.clone(),
            system_program: system_program::id(),
            cpi_program,
        };

        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        Ok(ctx)
    }
}
