use crate::state::VaultState;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // Vault account
    #[account(
        mut, // to modify its balance
        seeds = [b"vault", vault_state.key().as_ref()], // to get address
        bump = vault_state.vault_bump, // to get address
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut, // to modify its balance
        seeds = [b"state", user.key().as_ref()], // to validate address was created by signer
        bump = vault_state.state_bump, // to get address
        close = user,
    )]
    pub vault_state: Account<'info, VaultState>,

    system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        // Prepare transfer from vault to user
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        
        // Get seeds for vault address (NOT vault state)
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        // Prepare transfer context
        let cpi_ctx = CpiContext::new_with_signer(System::id(), cpi_accounts, signer_seeds);
        
        // transfer vault balance
        transfer(cpi_ctx, self.vault.lamports())?;

        Ok(())
    }
}
