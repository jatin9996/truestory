use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, TokenAccount, Mint};
use crate::services::oracle_integration::{get_current_sol_price};
use crate::tokenomics::{MAX_SUPPLY, INITIAL_LAUNCH_SUPPLY};
use crate::treasury_management::BurnFromTreasury;

#[derive(Accounts)]
pub struct BurnTokensContext<'info> {
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
    #[account(mut)]
    pub meme_token_state: Account<'info, MemeTokenState>,
    pub chainlink_feed: Account<'info, ChainlinkFeed>,
}

pub fn burn_tokens_based_on_price(ctx: Context<BurnTokensContext>, decrement: f64) -> Result<()> {
    let current_price = ctx.accounts.chainlink_feed.get_latest_price()?;
    let base_price = 1000.0; // Base price to start burning
    let price_drop_increment = 50.0;

    if current_price < base_price {
        let increments = ((base_price - current_price) / price_drop_increment).floor() as u64;
        let burn_amount = (MAX_SUPPLY / 100) * increments; // 1% of MAX_SUPPLY per increment

        if ctx.accounts.meme_token_state.total_supply - burn_amount < INITIAL_LAUNCH_SUPPLY {
            return Err(ErrorCode::Underflow.into());
        }

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.treasury.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::burn(cpi_ctx, burn_amount)?;

        // Decrease total supply when tokens are burned
        ctx.accounts.meme_token_state.total_supply -= burn_amount;
    }

    Ok(())
}