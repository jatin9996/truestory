use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn};

#[derive(Accounts)]
pub struct OracleIntegration {
    pub oracle: Account<'info, OracleAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>, // Ensure this account is included and properly initialized
    pub burn_authority: Signer<'info>, // This account must have the authority to burn tokens
}

pub fn update_oracle(ctx: Context<OracleIntegration>, new_price: u64) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;
    oracle.price = new_price;

    if new_price < 1000 {
        // Calculate how much to burn based on the price drop
        let drop = (1000 - new_price) / 50;
        let burn_amount = drop * (2 * oracle.treasury_supply / 100); // 2% of treasury for each $50 drop

        // Call burn function here
        burn_treasury_tokens(ctx, burn_amount)?;
    }

    Ok(())
}

// Function to burn tokens from the treasury
pub fn burn_treasury_tokens(ctx: Context<OracleIntegration>, amount: u64) -> Result<()> {
    let cpi_accounts = Burn {
        mint: ctx.accounts.treasury.to_account_info(),
        from: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.burn_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, amount)?;

    Ok(())
}

