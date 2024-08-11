use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Transaction exceeds the maximum allowed purchase limit.")]
    ExceedsMaxAllowedPurchase,
}
