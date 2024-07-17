use anchor_lang::prelude::*;
use anchor_spl::token::Mint; // Added this line

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 48)]
    pub meme_token_state: Account<'info, MemeTokenState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub mint: Account<'info, Mint>,
}

#[account]
pub struct MemeTokenState {
    pub max_supply: u64,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub launch_time: i64, // Unix timestamp of the launch time
    pub initialized: bool,
    pub in_progress: bool, // Added this field
}

#[error_code]
pub enum ErrorCode {
    #[msg("Already initialized.")]
    AlreadyInitialized,
    #[msg("Underflow occurred during calculation.")]
    Underflow,
    #[msg("Insufficient funds in the treasury.")]
    InsufficientFunds,
    #[msg("Overflow occurred during mint calculation.")]
    Overflow,
    #[msg("Max supply exceeded during mint calculation.")]
    MaxSupplyExceeded,
}

pub fn initialize(ctx: Context<Initialize>, decimals: u8) -> Result<()> {
    if ctx.accounts.meme_token_state.initialized {
        return Err(error!(ErrorCode::AlreadyInitialized));
    }
    ctx.accounts.meme_token_state.initialized = true;

    let meme_token_state = &mut ctx.accounts.meme_token_state;
    meme_token_state.max_supply = 2998944000000; // Maximum supply: 2,998,944,000,000
    meme_token_state.total_supply = 2998944000000; // Total supply: 2,998,944,000,000
    meme_token_state.circulating_supply = 599788800000; // Initial launch supply: 599,788,800,000 (20%)
    meme_token_state.launch_time = Clock::get()?.unix_timestamp; // Set launch time
    meme_token_state.in_progress = false; // Initialize the in_progress field
    ctx.accounts.mint.decimals = decimals;
    Ok(())
}