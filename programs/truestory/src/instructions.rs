use anchor_lang::prelude::*;
use bincode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Instruction {
    MintAndDistribute { mint_amount: u64 },
    Burn { sol_price: f64 },
}

impl From<&[u8]> for Instruction {
    fn from(slice: &[u8]) -> Self {
        bincode::deserialize(slice).unwrap_or_else(|_| panic!("Failed to deserialize Instruction"))
    }
}
