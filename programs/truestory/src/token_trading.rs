use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct BuyTokens {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub meme_token_state: Account<'info, MemeTokenState>,
}

pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
    // Transfer SOL from buyer to treasury
    let transfer_sol_cpi_accounts = Transfer {
        from: ctx.accounts.buyer.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info(),
    };
    let transfer_sol_cpi_program = ctx.accounts.system_program.to_account_info();
    let transfer_sol_cpi_ctx = CpiContext::new(transfer_sol_cpi_program, transfer_sol_cpi_accounts);
    anchor_spl::token::transfer(transfer_sol_cpi_ctx, amount)?;

    // Mint tokens to buyer's account
    let mint_cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.treasury.to_account_info(),
    };
    let mint_cpi_program = ctx.accounts.token_program.to_account_info();
    let seeds = &[b"treasury".as_ref(), &[ctx.accounts.treasury.bump]];
    let signer = &[&seeds[..]];
    let mint_cpi_ctx = CpiContext::new_with_signer(mint_cpi_program, mint_cpi_accounts, signer);
    token::mint_to(mint_cpi_ctx, amount)?;

    // Update circulating supply
    ctx.accounts.meme_token_state.circulating_supply += amount;

    Ok(())
}

#[derive(Accounts)]
pub struct SellTokens {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub meme_token_state: Account<'info, MemeTokenState>,
}

pub fn sell_tokens(ctx: Context<SellTokens>, amount: u64) -> Result<()> {
    // Burn tokens from the seller's token account
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.seller_token_account.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, amount)?;

    // Transfer SOL from the treasury to the seller
    let seeds = &[
        b"treasury".as_ref(),
        &[ctx.accounts.treasury.bump],
    ];
    let signer = &[&seeds[..]];
    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury.to_account_info(),
        to: ctx.accounts.seller.to_account_info(),
        authority: ctx.accounts.treasury.to_account_info(), 
    };
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    anchor_spl::token::transfer(cpi_ctx, amount)?;

    // Update circulating supply
    ctx.accounts.meme_token_state.circulating_supply -= amount;

    Ok(())
}
