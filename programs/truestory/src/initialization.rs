use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize {
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
}

pub fn initialize(ctx: Context<Initialize>, max_supply: u64, total_supply: u64, circulating_supply: u64, decimals: u8) -> Result<()> {
    if ctx.accounts.meme_token_state.initialized {
        return Err(error!(ErrorCode::AlreadyInitialized));
    }
    ctx.accounts.meme_token_state.initialized = true;

    let meme_token_state = &mut ctx.accounts.meme_token_state;
    meme_token_state.max_supply = 3125174400000; // Maximum supply: 3,125,174,400,000
    meme_token_state.total_supply = 3125174400000; // Total supply: 3,125,174,400,000
    meme_token_state.circulating_supply = 625034880000; // Circulating supply: 625,034,880,000
    ctx.accounts.mint.decimals = decimals;
    Ok(())
}