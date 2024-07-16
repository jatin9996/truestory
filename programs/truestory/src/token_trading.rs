use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer, Burn};
use chainlink_solana::ChainlinkFeed; // Add this line

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
    pub chainlink_feed: Account<'info, ChainlinkFeed>, // Add this line
}

#[error_code]
pub enum ErrorCode {
    #[msg("Failed to transfer funds.")]
    TransferFailed,
    #[msg("Failed to mint tokens.")]
    MintFailed,
    #[msg("Failed to burn tokens.")]
    BurnFailed,
    #[msg("Failed to transfer tax to treasury.")]
    TaxTransferFailed,
    #[msg("Overflow occurred during calculation.")]
    Overflow,
    #[msg("Already initialized.")]
    AlreadyInitialized,
    #[msg("Underflow occurred during calculation.")]
    Underflow,
    #[msg("Insufficient balance for rewards.")]
    InsufficientBalance,
    #[msg("Unauthorized attempt to sell tokens.")]
    Unauthorized,
}

pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
    let transfer_result = anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.treasury.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            }
        ),
        amount
    ).map_err(|_| error!(ErrorCode::TransferFailed))?;

    let mint_cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.treasury.to_account_info(),
    };
    let mint_cpi_program = ctx.accounts.token_program.to_account_info();
    let seeds = &[b"treasury".as_ref(), &[ctx.accounts.treasury.bump]];
    let signer = &[&seeds[..]];
    let mint_cpi_ctx = CpiContext::new_with_signer(mint_cpi_program, mint_cpi_accounts, signer);
    token::mint_to(mint_cpi_ctx, amount).map_err(|_| error!(ErrorCode::MintFailed))?;

    ctx.accounts.meme_token_state.circulating_supply = ctx.accounts.meme_token_state.circulating_supply.checked_add(amount).ok_or(error!(ErrorCode::Overflow))?;

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
    pub chainlink_feed: Account<'info, ChainlinkFeed>, // Add this line
}

pub fn sell_tokens(ctx: Context<SellTokens>, amount: u64) -> Result<()> {
    require!(ctx.accounts.seller.to_account_info().is_signer, ErrorCode::Unauthorized);

    // Prevent reentrancy by marking the state as in-progress
    ctx.accounts.meme_token_state.in_progress = true;

    let current_time = Clock::get()?.unix_timestamp;
    let elapsed_hours = (current_time - ctx.accounts.meme_token_state.launch_time) / 3600;
    let tax_rate = if elapsed_hours < 100 { 100 - elapsed_hours } else { 0 };
    let tax_amount = amount * tax_rate / 100;
    let net_amount = amount.checked_sub(tax_amount).ok_or(ErrorCode::Underflow)?;

    let burn_cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.seller_token_account.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    let burn_cpi_program = ctx.accounts.token_program.to_account_info();
    let burn_cpi_ctx = CpiContext::new(burn_cpi_program, burn_cpi_accounts);
    token::burn(burn_cpi_ctx, net_amount).map_err(|_| error!(ErrorCode::BurnFailed))?;

    let transfer_cpi_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    let transfer_cpi_program = ctx.accounts.token_program.to_account_info();
    let transfer_cpi_ctx = CpiContext::new(transfer_cpi_program, transfer_cpi_accounts);
    anchor_spl::token::transfer(transfer_cpi_ctx, tax_amount).map_err(|_| error!(ErrorCode::TaxTransferFailed))?;

    // Update circulating supply
    ctx.accounts.meme_token_state.circulating_supply = ctx.accounts.meme_token_state.circulating_supply.checked_sub(amount).ok_or(ErrorCode::Underflow)?;

    // Mark the operation as complete
    ctx.accounts.meme_token_state.in_progress = false;

    Ok(())
}