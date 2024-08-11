use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

pub const INITIAL_TAX_RATE: u64 = 100; // 100%
pub const TAX_DECREMENT: u64 = 1; // 1% per hour
pub const TAX_DURATION_HOURS: u64 = 100;

pub fn calculate_tax_rate(start_time: i64) -> u64 {
    let current_time = Clock::get().unwrap().unix_timestamp;
    let hours_elapsed = (current_time - start_time) / 3600;

    if hours_elapsed >= TAX_DURATION_HOURS {
        0
    } else {
        INITIAL_TAX_RATE - (hours_elapsed as u64 * TAX_DECREMENT)
    }
}
