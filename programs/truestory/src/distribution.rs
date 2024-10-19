use anchor_lang::prelude::*;

pub const TEAM_PERCENTAGE: u64 = 25; // 0.25%
pub const AIRDROP_PERCENTAGE: u64 = 125; // 0.125%
pub const TREASURY_PERCENTAGE: u64 = 500; // 0.5%

pub fn distribute_tokens(mint_amount: u64) -> (u64, u64, u64) {
    let team_share = mint_amount * TEAM_PERCENTAGE / 10000;
    let airdrop_share = mint_amount * AIRDROP_PERCENTAGE / 100000;
    let treasury_share = mint_amount * TREASURY_PERCENTAGE / 100000;
    (team_share, airdrop_share, treasury_share)
}
