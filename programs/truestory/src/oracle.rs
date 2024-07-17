use anchor_lang::prelude::*;

#[account]
pub struct OracleAccount {
    pub price: u64,
    pub treasury_supply: u64, // Added this field
    // other fields...
}