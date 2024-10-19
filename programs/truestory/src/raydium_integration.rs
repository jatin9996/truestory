use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use solana_program::pubkey::Pubkey;
use crate::raydium_constants::*;

#[derive(Accounts)]
pub struct RaydiumPool<'info> {
    #[account(mut)]
    pub tsm_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sol_token_account: Account<'info, TokenAccount>,
    pub pool_program: Program<'info, Token>,
    #[account(seeds = [tsm_token_account.key().as_ref()], bump)]
    pub pool_authority: Signer<'info>,
    // Add this line to store the bump seed
    pub pool_authority_bump: u8,
}

pub fn create_liquidity_pool(ctx: Context<RaydiumPool>, initial_tsm: u64, initial_sol: u64) -> Result<()> {
    // Logic to create liquidity pool on Raydium
    let seeds = &[ctx.accounts.pool_authority.to_account_info().key.as_ref(), &[ctx.accounts.pool_authority.to_account_info().bump]];
    let signer = &[&seeds[..]];

    // Call to Raydium's create liquidity pool function, assuming it's exposed via an external program
    let result = raydium_sdk::create_pool(
        &ctx.accounts.pool_program.to_account_info(),
        &ctx.accounts.tsm_token_account.to_account_info(),
        &ctx.accounts.sol_token_account.to_account_info(),
        initial_tsm,
        initial_sol,
        signer,
    );

    result.map_err(|e| e.into())
}

pub fn add_liquidity(ctx: Context<RaydiumPool>, tsm_amount: u64, sol_amount: u64) -> Result<()> {
    // Logic to add liquidity to Raydium pool
    let seeds = &[ctx.accounts.pool_authority.to_account_info().key.as_ref(), &[ctx.accounts.pool_authority.to_account_info().bump]];
    let signer = &[&seeds[..]];

    // Call to Raydium's add liquidity function, assuming it's exposed via an external program
    let result = raydium_sdk::add_liquidity(
        &ctx.accounts.pool_program.to_account_info(),
        &ctx.accounts.tsm_token_account.to_account_info(),
        &ctx.accounts.sol_token_account.to_account_info(),
        tsm_amount,
        sol_amount,
        signer,
    );

    result.map_err(|e| e.into())
}

pub fn remove_liquidity(ctx: Context<RaydiumPool>, tsm_amount: u64, sol_amount: u64) -> Result<()> {
    // Logic to remove liquidity from Raydium pool
    let seeds = &[ctx.accounts.pool_authority.to_account_info().key.as_ref(), &[ctx.accounts.pool_authority.to_account_info().bump]];
    let signer = &[&seeds[..]];

    // Call to Raydium's remove liquidity function, assuming it's exposed via an external program
    let result = raydium_sdk::remove_liquidity(
        &ctx.accounts.pool_program.to_account_info(),
        &ctx.accounts.tsm_token_account.to_account_info(),
        &ctx.accounts.sol_token_account.to_account_info(),
        tsm_amount,
        sol_amount,
        signer,
    );

    result.map_err(|e| e.into())
}

pub fn swap_tokens(ctx: Context<RaydiumPool>, amount: u64, from_token: Pubkey, to_token: Pubkey) -> Result<()> {
    // Logic to swap tokens using Raydium
    let seeds = &[ctx.accounts.pool_authority.to_account_info().key.as_ref(), &[ctx.accounts.pool_authority.to_account_info().bump]];
    let signer = &[&seeds[..]];

    // Assuming a swap function exists in the Raydium SDK
    let result = raydium_sdk::swap(
        &ctx.accounts.pool_program.to_account_info(),
        &from_token,
        &to_token,
        amount,
        signer,
    );

    result.map_err(|e| e.into())
}

pub fn fetch_price_data() -> Result<f64> {
    // Assuming there's a function in the Raydium SDK to fetch price data from the Oracle
    let price = raydium_sdk::get_pool_price(STANDARD_AMM_ID).map_err(|e| e.into())?;
    Ok(price)
}

pub fn stake_tokens(ctx: Context<RaydiumPool>, amount: u64) -> Result<()> {
    // Logic to stake tokens in Raydium pool
    let seeds = &[ctx.accounts.pool_authority.to_account_info().key.as_ref(), &[ctx.accounts.pool_authority.to_account_info().bump]];
    let signer = &[&seeds[..]];

    // Assuming a staking function exists in the Raydium SDK
    let result = raydium_sdk::stake(
        &ctx.accounts.pool_program.to_account_info(),
        &ctx.accounts.tsm_token_account.to_account_info(),
        amount,
        signer,
    );

    result.map_err(|e| e.into())
}

pub fn setup_farming_rewards(ctx: Context<RaydiumPool>, amount: u64) -> Result<()> {
    // Setup farming rewards using Raydium
    let seeds = &[ctx.accounts.pool_authority.to_account_info().key.as_ref(), &[ctx.accounts.pool_authority.to_account_info().bump]];
    let signer = &[&seeds[..]];

    // Assuming a function to setup farming rewards exists in the Raydium SDK
    let result = raydium_sdk::setup_farming_rewards(
        &ctx.accounts.pool_program.to_account_info(),
        amount,
        signer,
    );

    result.map_err(|e| e.into())
}

pub fn participate_in_ido(ctx: Context<RaydiumPool>, amount: u64) -> Result<()> {
    // Participate in IDO on Raydium
    let seeds = &[ctx.accounts.pool_authority.to_account_info().key.as_ref(), &[ctx.accounts.pool_authority.to_account_info().bump]];
    let signer = &[&seeds[..]];

    // Assuming a function to participate in IDO exists in the Raydium SDK
    let result = raydium_sdk::participate_in_ido(
        &ctx.accounts.pool_program.to_account_info(),
        amount,
        signer,
    );

    result.map_err(|e| e.into())
}
