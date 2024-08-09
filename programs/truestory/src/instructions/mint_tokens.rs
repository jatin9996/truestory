use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::services::coingecko::get_sol_price;
use crate::tokenomics::{calculate_mint_amount, MAX_SUPPLY, INITIAL_LAUNCH_SUPPLY};
use anchor_lang::prelude::*;

pub const MAX_ALLOWED_PURCHASE: u64 = (MAX_SUPPLY / 400) as u64; // 0.25% of MAX_SUPPLY

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let token_program = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let owner = next_account_info(account_info_iter)?;

    let current_price = get_sol_price()?;
    let base_price = 200.0; // Starting price to check increments
    let price_increment = 5.0;

    let mint_amount = calculate_mint_amount(current_price, base_price, price_increment);
    if mint_amount > 0 {
        msg!("Minting {} tokens due to price increase", mint_amount);
        // Implement the actual minting logic here
    }

    // Limit wallet purchases (anti-whale)
    use crate::instructions::utils::limit_wallet_purchase;
    use crate::tokenomics::{calculate_mint_amount, MAX_ALLOWED_PURCHASE};

    limit_wallet_purchase(mint_account, MAX_ALLOWED_PURCHASE)?;

    Ok(())
}

#[derive(Accounts)]
pub struct MintAdditionalSupply<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn limit_wallet_purchase(
    purchaser: &AccountInfo,
    max_allowed: u64,
) -> ProgramResult {
    // Check if the purchase exceeds the max_allowed
    if purchaser.lamports.borrow() > max_allowed {
        return Err(ProgramError::Custom(1));
    }

    // If it does, throw an error

    Ok(())
}