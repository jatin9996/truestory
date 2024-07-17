use anchor_lang::prelude::*;

#[account]
pub struct OracleAccount {
    pub price: u64,
    // other fields...
}