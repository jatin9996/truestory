use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, TokenAccount, Mint};

#[derive(Accounts)]
pub struct BurnFromTreasury {
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>, // Mint account of the token
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>, // Include the token program
}

pub fn burn_treasury_tokens(ctx: Context<BurnFromTreasury>, amount: u64) -> Result<()> {
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, amount)?;

    Ok(())
}