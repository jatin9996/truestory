use anchor_lang::prelude::*;

pub mod initialization;
pub mod token_minting;
pub mod oracle;
pub mod treasury_management;
pub mod token_trading;
pub mod rewards;
pub mod oracle_integration;
pub mod chainlink_feed; // Ensure this line is present to declare the chainlink_feed module

use initialization::{initialize, MemeTokenState};
use token_minting::mint_tokens;
use oracle_integration::update_oracle;
use treasury_management::burn_treasury_tokens;

declare_id!("3dupjHU543SdKpSkdTyPSbPLAowgTPRT15jG2rJd9fD1");

#[program]
pub mod truestory_meme {
    use super::*;

    pub use initialization::initialize;
    pub use token_minting::mint_tokens;
    pub use oracle_integration::update_oracle;
    pub use treasury_management::burn_treasury_tokens;
    pub use token_trading::{buy_tokens, sell_tokens};
    pub use rewards::reward_users;
}