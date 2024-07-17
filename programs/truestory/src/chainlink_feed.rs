use anchor_lang::prelude::*;

#[account]
pub struct ChainlinkFeed {
    pub price: u64,
    // other fields...
}

impl ChainlinkFeed {
    pub fn get_price(&self) -> Result<u64> {
        Ok(self.price)
    }
}