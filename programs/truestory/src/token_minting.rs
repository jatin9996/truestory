use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, MintTo};
use chainlink_solana::ChainlinkFeed; // Add this line

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
    pub to_community: Account<'info, TokenAccount>, // Add this line
    #[account(mut)]
    pub to_advisors: Account<'info, TokenAccount>,
    pub chainlink_feed: Account<'info, ChainlinkFeed>, // Add this line
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
    let increments = (oracle.price - 200) / 5; // Changed from 50 to 5
    let additional_mint = (increments * meme_token_state.max_supply) / 100; // 1% for each $5 increment

    let total_mint = base_amount.checked_add(additional_mint).ok_or(error!(ErrorCode::Overflow))?;
    if meme_token_state.total_supply + total_mint > meme_token_state.max_supply {
        return Err(error!(ErrorCode::MaxSupplyExceeded));
    }

    distribute_tokens(ctx, total_mint)?;
    Ok(())
}

fn distribute_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let treasury_amount = (amount * 400) / 1000; // 40%
    let community_amount = (amount * 300) / 1000; // 30%
    let airdrop_amount = (amount * 100) / 1000; // 10%
    let advisors_amount = (amount * 100) / 1000; // 10%
    let team_amount = (amount * 100) / 1000; // 10%

    token::mint_to(ctx.accounts.to_treasury.to_account_info(), treasury_amount)?;
    token::mint_to(ctx.accounts.to_community.to_account_info(), community_amount)?;
    token::mint_to(ctx.accounts.to_airdrops.to_account_info(), airdrop_amount)?;
    token::mint_to(ctx.accounts.to_advisors.to_account_info(), advisors_amount)?;
    token::mint_to(ctx.accounts.to_team.to_account_info(), team_amount)?;

    ctx.accounts.meme_token_state.total_supply += amount;
    ctx.accounts.meme_token_state.circulating_supply += amount;
    Ok(())
}