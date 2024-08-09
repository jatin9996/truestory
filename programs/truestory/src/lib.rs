use anchor_lang::prelude::*;

pub mod initialize;
pub mod mint_tokens;
pub mod oracle;
pub mod treasury_management;
pub mod token_trading;
pub mod rewards;
pub mod oracle_integration;
pub mod chainlink_feed; // Ensure this line is present to declare the chainlink_feed module
pub mod instructions; // Assuming you have a separate module for handling different instructions

use initialization::{initialize, MemeTokenState};
use token_minting::{mint_tokens, MintAdditionalSupply}; // Import additional structs if needed
use oracle_integration::update_oracle;
use treasury_management::burn_treasury_tokens;
use instructions::{transfer_tokens, mint_tokens as instructions_mint_tokens}; // Example if you have separate instruction handling

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
    pub use instructions::{transfer_tokens, instructions_mint_tokens}; // Use the instruction handlers
}