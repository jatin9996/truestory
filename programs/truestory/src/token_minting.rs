use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, MintTo};

#[derive(Accounts)]
pub struct MintTokens {
    #[account(mut)]
    pub meme_token_state: Account<'info, MemeTokenState>,
    #[account(mut)]
    pub oracle: Account<'info, OracleAccount>, // Oracle account to get the current price
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to_team: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_airdrops: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_marketing: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_advisors: Account<'info, TokenAccount>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow occurred during mint calculation.")]
    Overflow,
    #[msg("Max supply exceeded during mint calculation.")]
    MaxSupplyExceeded,
}

pub fn mint_tokens(ctx: Context<MintTokens>, base_amount: u64) -> Result<()> {
    let meme_token_state = &mut ctx.accounts.meme_token_state;
    let oracle = &ctx.accounts.oracle;
    let increments = (oracle.price - 200) / 5;
    let additional_mint = (increments * meme_token_state.max_supply) / 100; // 1% for each $5 increment

    let total_mint = base_amount.checked_add(additional_mint).ok_or(error!(ErrorCode::Overflow))?;
    if meme_token_state.total_supply + total_mint > meme_token_state.max_supply {
        return Err(error!(ErrorCode::MaxSupplyExceeded));
    }

    distribute_tokens(ctx, total_mint)?;
    Ok(())
}

fn distribute_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let treasury_amount = (amount * 375) / 1000;
    let marketing_amount = (amount * 125) / 1000;
    let airdrop_amount = (amount * 125) / 1000;
    let advisors_amount = (amount * 125) / 1000;
    let team_amount = (amount * 125) / 1000;
    let circulating_amount = (amount * 125) / 1000;

    token::mint_to(ctx.accounts.to_treasury.to_account_info(), treasury_amount)?;
    token::mint_to(ctx.accounts.to_marketing.to_account_info(), marketing_amount)?;
    token::mint_to(ctx.accounts.to_airdrops.to_account_info(), airdrop_amount)?;
    token::mint_to(ctx.accounts.to_advisors.to_account_info(), advisors_amount)?;
    token::mint_to(ctx.accounts.to_team.to_account_info(), team_amount)?;

    ctx.accounts.meme_token_state.total_supply += amount;
    ctx.accounts.meme_token_state.circulating_supply += circulating_amount;
    Ok(())
}
