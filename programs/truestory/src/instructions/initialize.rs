use anchor_lang::prelude::*;
use crate::state::TokenAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub token_account: ProgramAccount<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<Initialize>, initial_supply: u64) -> Result<()> {
    let token_account = &mut ctx.accounts.token_account;
    token_account.supply = initial_supply;
    token_account.authority = ctx.accounts.authority.key();
    Ok(())
}
