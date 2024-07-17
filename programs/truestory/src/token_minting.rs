use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, MintTo};
use crate::chainlink_feed::ChainlinkFeed; // Updated import path
use crate::oracle::OracleAccount;
use crate::initialization::MemeTokenState;

#[derive(Accounts)]
pub struct MintTokens<'info> {
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
    pub token_program: Program<'info, token::Token>, // Add this line
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

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let mint_to_treasury_ctx = CpiContext::new(
        cpi_program.clone(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to_treasury.to_account_info(),
            authority: ctx.accounts.meme_token_state.to_account_info(),
        },
    );
    token::mint_to(mint_to_treasury_ctx, treasury_amount)?;

    let mint_to_community_ctx = CpiContext::new(
        cpi_program.clone(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to_community.to_account_info(),
            authority: ctx.accounts.meme_token_state.to_account_info(),
        },
    );
    token::mint_to(mint_to_community_ctx, community_amount)?;

    let mint_to_airdrops_ctx = CpiContext::new(
        cpi_program.clone(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to_airdrops.to_account_info(),
            authority: ctx.accounts.meme_token_state.to_account_info(),
        },
    );
    token::mint_to(mint_to_airdrops_ctx, airdrop_amount)?;

    let mint_to_advisors_ctx = CpiContext::new(
        cpi_program.clone(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to_advisors.to_account_info(),
            authority: ctx.accounts.meme_token_state.to_account_info(),
        },
    );
    token::mint_to(mint_to_advisors_ctx, advisors_amount)?;

    let mint_to_team_ctx = CpiContext::new(
        cpi_program,
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to_team.to_account_info(),
            authority: ctx.accounts.meme_token_state.to_account_info(),
        },
    );
    token::mint_to(mint_to_team_ctx, team_amount)?;

    ctx.accounts.meme_token_state.total_supply += amount;
    ctx.accounts.meme_token_state.circulating_supply += amount;
    Ok(())
}