use anchor_lang::prelude::*;

declare_id!("3dupjHU543SdKpSkdTyPSbPLAowgTPRT15jG2rJd9fD1");

#[program]
pub mod truestory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
