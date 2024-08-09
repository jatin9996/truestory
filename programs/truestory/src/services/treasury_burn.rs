use anchor_lang::prelude::*;

pub const SOL_TRIGGER_PRICE: f64 = 1000.0;
pub const SOL_DROP_PRICE: f64 = 5.0;
pub const BURN_PERCENTAGE: u64 = 2; // 2% per $5 drop below $1000

pub fn burn_treasury_tokens(current_sol_price: f64, treasury_amount: u64) -> u64 {
    if current_sol_price < SOL_TRIGGER_PRICE {
        let drops = ((SOL_TRIGGER_PRICE - current_sol_price) / SOL_DROP_PRICE).floor() as u64;
        let burn_amount = treasury_amount * BURN_PERCENTAGE * drops / 100;
        burn_amount
    } else {
        0
    }
}