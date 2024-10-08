use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, TokenAccount, Mint};
use crate::chainlink_feed::ChainlinkFeed; // Updated import path
use crate::initialization::MemeTokenState; // Import MemeTokenState from the correct module

#[derive(Accounts)]
pub struct BurnFromTreasury<'info> {
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>, // Mint account of the token
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>, // Include the token program
    #[account(mut)]
    pub meme_token_state: Account<'info, MemeTokenState>,
    pub chainlink_feed: Account<'info, ChainlinkFeed>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in the treasury.")]
    InsufficientFunds,
    #[msg("Underflow occurred during calculation.")]
    Underflow,
    // other error codes...
}

pub fn burn_treasury_tokens(ctx: Context<BurnFromTreasury>, amount: u64) -> Result<()> {
    if amount > ctx.accounts.treasury.amount {
        return Err(error!(ErrorCode::InsufficientFunds));
    }

    if ctx.accounts.meme_token_state.total_supply < amount {
        return Err(error!(ErrorCode::Underflow));
    }

    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, amount)?;

    // Decrease total supply when tokens are burned
    ctx.accounts.meme_token_state.total_supply -= amount;

    Ok(())
}