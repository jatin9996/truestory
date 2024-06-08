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

pub fn initialize(ctx: Context<Initialize>, max_supply: u64, decimals: u8) -> Result<()> {
    let meme_token_state = &mut ctx.accounts.meme_token_state;
    meme_token_state.max_supply = max_supply;
    meme_token_state.total_supply = 0;
    ctx.accounts.mint.decimals = decimals;
    Ok(())
}