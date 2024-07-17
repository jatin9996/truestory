use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, TokenAccount};

#[derive(Accounts)]
pub struct RewardUsers<'info> {
    #[account(mut)]
    pub reward_recipient: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance to issue rewards.")]
    InsufficientBalance,
    #[msg("Reward limit exceeded.")]
    RewardLimitExceeded,
}

pub fn reward_users(ctx: Context<RewardUsers>, amount: u64) -> Result<()> {
    if amount > MAX_REWARD_LIMIT {
        return Err(error!(ErrorCode::RewardLimitExceeded));
    }

    if !can_issue_rewards(amount) {
        return Err(error!(ErrorCode::InsufficientBalance));
    }

    // Mint tokens directly to the user's account as a reward
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.reward_recipient.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, amount)?;

    Ok(())
}

// Function to check if rewards can be issued
fn can_issue_rewards(amount: u64) -> bool {
    // Implement your logic to check if rewards can be issued
    true
}