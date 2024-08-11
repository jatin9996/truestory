use anchor_lang::prelude::*;
use solana_program::{
    account_info::{AccountInfo},
    entrypoint::ProgramResult,
    msg,
};

#[derive(Accounts)]
pub struct DistributeMintedTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub team_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub airdrop_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn distribute_minted_tokens(
    accounts: &[AccountInfo],
    mint_amount: u64,
) -> ProgramResult {
    let ctx = Context::new(accounts);

    let team_share = mint_amount * 25 / 10000; // 0.25% of minted
    let airdrop_share = mint_amount * 125 / 100000; // 0.125% of minted

    // Logic to mint to team wallet
    msg!("Distributing {} tokens to team wallet", team_share);
    {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.team_wallet.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, team_share)?;
    }

    // Logic to mint to airdrop wallet
    msg!("Distributing {} tokens to airdrop wallet", airdrop_share);
    {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.airdrop_wallet.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, airdrop_share)?;
    }

    // Remaining tokens go to the treasury
    let treasury_share = mint_amount - (team_share + airdrop_share);
    msg!("Distributing {} tokens to treasury", treasury_share);
    {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.treasury_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, treasury_share)?;
    }

    Ok(())
}