use anchor_lang::prelude::*;

declare_id!("Fg6PaFp2KH8HvdxHV4NHq6GVpA3DQ4YVnVnXZWycuZXs");

#[account]
pub struct TokenAccount {
    pub authority: Pubkey,
    pub supply: u64,
    pub minted_supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
}

impl TokenAccount {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 1;
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = TokenAccount::LEN)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: AccountInfo,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_authority: AccountInfo,
}

#[derive(Accounts)]
pub struct Recover<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recover_authority: AccountInfo,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub burn_authority: AccountInfo,
}
