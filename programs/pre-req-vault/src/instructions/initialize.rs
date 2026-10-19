use crate::state::VaultState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Signs the transaction and pays for account creation
    #[account(mut)]
    pub user: Signer<'info>,

    // Stores vault data, including both bump values
    #[account(
        init, // Create the account
        payer = user, // pay creation with signer balance
        seeds = [b"state", user.key().as_ref()], // seed to generate new address (state,user_Address)
        bump, // Addres will not have PK
        space = 8 + VaultState::INIT_SPACE // Space required by new address
    )]
    pub vault_state: Account<'info, VaultState>,

    // separate PDA owned by the System Program
    // Not initialized
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    // InitializeBumps: the bump values found for the accounts that use PDA constraints.
    // Idea: Easy to access them
    pub fn intialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // Save data to state
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}
