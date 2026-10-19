// Import module
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

// Re-export module to be able to re-used again in inherited module
pub use constants::*;
pub use instructions::*;
pub use state::*;

// Where program will be deployed
declare_id!("FVBqAhYiu7UCzmpEkUUJSkc5mFGtrhdK8D3hf35Fjf5o");


#[program]
pub mod pre_req_vault {
    use super::*;

    // Initialize
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.intialize(&ctx.bumps)
    }

    // deposit funds
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    // withdraw funds
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, github_user: String) -> Result<()> {
        ctx.accounts.withdraw(amount, &github_user)
    }

    // close
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
