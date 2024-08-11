use anchor_lang::prelude::*;

// Declare all modules that contain instruction handlers or relevant functionality
pub mod initialization; // Added this line to declare the initialization module
pub mod distribution; // New module for token distribution logic
pub mod tax; // New module for dynamic tax rate logic

pub mod raydium_integration; // Include the new Raydium integration module

// Import necessary structs and functions from the modules
use initialization::{initialize, MemeTokenState};
use token_minting::{mint_tokens, MintAdditionalSupply}; // Import additional structs if needed
use oracle_integration::update_oracle;
use treasury_management::burn_treasury_tokens;
use instructions::{transfer_tokens, mint_tokens as instructions_mint_tokens, mint_tokens_based_on_price, transfer}; // Example if you have separate instruction handling
use burn_tokens::burn_tokens_based_on_price; // Use the new function
use distribution::{distribute_tokens}; // Import distribute_tokens function from distribution module
use tax::{calculate_tax_rate}; // Import calculate_tax_rate function from tax module
use treasury_burn::{burn_treasury_tokens}; // Import burn_treasury_tokens function from treasury_burn module
use raydium_integration::*; // Import functions from the Raydium integration module

declare_id!("3dupjHU543SdKpSkdTyPSbPLAowgTPRT15jG2rJd9fD1");

pub mod services {
    pub mod treasury_burn;
}

use services::treasury_burn::burn_based_on_sol_price;

#[program]
pub mod truestory_meme {
    use super::*;

    // Use the imported functions and structs in the program macro
    pub use initialization::initialize;
    pub use mint_tokens::mint_tokens;
    pub use oracle_integration::update_oracle;
    pub use treasury_burns::burn_treasury_tokens;
    pub use token_trading::{buy_tokens, sell_tokens};
    pub use rewards::reward_users;
    pub use instructions::{transfer_tokens, instructions_mint_tokens, mint_tokens_based_on_price, transfer}; // Use the instruction handlers
    pub use burn_tokens::burn_tokens_based_on_price; // Use the new function

    pub fn execute_mint_and_distribute(ctx: Context<MintAdditionalSupply>, mint_amount: u64) -> Result<()> {
        let (team_share, airdrop_share, treasury_share) = distribute_tokens(mint_amount);
        // Logic to mint and distribute to respective wallets
        Ok(())
    }

    pub fn execute_burn(ctx: Context<BurnFromTreasury>, sol_price: f64) -> Result<()> {
        let treasury_amount = ctx.accounts.treasury.amount;
        let burn_amount = burn_treasury_tokens(sol_price, treasury_amount);
        burn_based_on_sol_price(ctx, sol_price, burn_amount)
    }
}