use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Transfer, TokenAccount};
use crate::chainlink_feed::ChainlinkFeed; // Updated import path
use crate::oracle::OracleAccount; // Import OracleAccount from the correct module

#[derive(Accounts)]
pub struct OracleIntegration<'info> {
    pub oracle: Account<'info, OracleAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>, // Ensure this account is included and properly initialized
    #[account(mut)]
    pub market_reserve: Account<'info, TokenAccount>, // Market reserve account for buying tokens
    pub burn_authority: Signer<'info>, // This account must have the authority to burn tokens
    pub token_program: Program<'info, token::Token>,
    pub chainlink_feed: Account<'info, ChainlinkFeed>,
}

const MIN_PRICE: u64 = 100; // Define this constant
const MAX_PRICE: u64 = 2000; // Define this constant

pub fn update_oracle(ctx: Context<OracleIntegration>) -> Result<()> {
    let new_price = ctx.accounts.chainlink_feed.get_price()?; // Fetch price from Chainlink

    if new_price < MIN_PRICE || new_price > MAX_PRICE {
        return Err(error!(ErrorCode::PriceOutOfRange));
    }

    let oracle = &mut ctx.accounts.oracle;
    oracle.price = new_price;

    if new_price >= 1050 {
        let drop = (new_price - 1050) / 5; // Changed from 50 to 5
        let spend_amount = drop * (15 * ctx.accounts.treasury.amount / 1000); // 1.5% for each $5 increment
        buy_and_burn_tokens(ctx, spend_amount)?;
    }

    if new_price < 1000 {
        // Calculate how much to burn based on the price drop
        let drop = (1000 - new_price) / 5; // Changed from 50 to 5
        let burn_amount = drop * (2 * oracle.treasury_supply / 100); // 2% of treasury for each $5 drop

        // Call burn function here
        burn_treasury_tokens(ctx, burn_amount)?;
    }

    Ok(())
}

// Function to burn tokens from the treasury
pub fn burn_treasury_tokens(ctx: Context<OracleIntegration>, amount: u64) -> Result<()> {
    let cpi_accounts = Burn {
        mint: ctx.accounts.treasury.to_account_info(),
        from: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.burn_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, amount)?;

    Ok(())
}

fn buy_and_burn_tokens(ctx: Context<OracleIntegration>, amount: u64) -> Result<()> {
    // Transfer tokens from market reserve to treasury
    let transfer_cpi_accounts = Transfer {
        from: ctx.accounts.market_reserve.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.market_reserve.to_account_info(),
    };
    let transfer_cpi_program = ctx.accounts.token_program.to_account_info();
    let transfer_cpi_ctx = CpiContext::new(transfer_cpi_program, transfer_cpi_accounts);
    token::transfer(transfer_cpi_ctx, amount)?;

    // Burn tokens from the treasury
    let burn_cpi_accounts = Burn {
        mint: ctx.accounts.treasury.to_account_info(),
        from: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.burn_authority.to_account_info(),
    };
    let burn_cpi_program = ctx.accounts.token_program.to_account_info();
    let burn_cpi_ctx = CpiContext::new(burn_cpi_program, burn_cpi_accounts);
    token::burn(burn_cpi_ctx, amount)?;

    Ok(())
}