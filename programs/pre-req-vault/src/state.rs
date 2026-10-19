use anchor_lang::prelude::*;

#[derive(InitSpace)] // Calculate required bytes to stor VaultState
#[account] // Implement trait to store this values: https://github.com/otter-sec/anchor/blob/d5da538301a8d9f7ccabd46b6a4028d0c15e9e4a/lang/attribute/account/src/lib.rs#L23
pub struct VaultState {
    pub vault_bump: u8, // ensure PDA is not a point in ed25519
    pub state_bump: u8, // ensure PDA is not a point in ed25519
}
