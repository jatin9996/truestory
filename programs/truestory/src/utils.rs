use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
};
use crate::tokenomics::{MAX_ALLOWED_PURCHASE, MAX_SUPPLY};

pub fn limit_wallet_purchase(
    purchaser: &AccountInfo,
    max_allowed: u64,
) -> ProgramResult {
    let balance = purchaser.lamports();

    if balance > max_allowed {
        return Err(ProgramError::Custom(1)); // Define a specific error code in your error module
    }

    Ok(())
}