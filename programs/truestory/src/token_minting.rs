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
}

pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let meme_token_state = &mut ctx.accounts.meme_token_state;
    let oracle = &ctx.accounts.oracle;

    // Calculate additional mint based on SOLANA price
    if oracle.price >= 200 {
        let price_increase = oracle.price - 200;
        let increments = price_increase / 5;
        let additional_mint = (increments * meme_token_state.max_supply) / 100; // 1% for each $5 increment

        let total_mint = amount + additional_mint;
        let team_amount = total_mint / 4;
        let airdrop_amount = total_mint / 8;
        let treasury_amount = total_mint / 2;

        if meme_token_state.total_supply + total_mint <= meme_token_state.max_supply {
            token::mint_to(ctx.accounts.to_team.to_account_info(), team_amount)?;
            token::mint_to(ctx.accounts.to_airdrops.to_account_info(), airdrop_amount)?;
            token::mint_to(ctx.accounts.to_treasury.to_account_info(), treasury_amount)?;

            meme_token_state.total_supply += total_mint;
        } else {
            return Err(ErrorCode::MaxSupplyReached.into());
        }
    }

    Ok(())
}