# Solana vault
Deploy porgrams in local net: `solana program deploy target/deploy/pre_req_vault.so --program-id target/deploy/pre_req_vault-keypair.json -u localhost`

Run `anchor keys sync` before running test

# General idea
* Anyone can deploy their own vault
* Users can only deposit into  withdraw from and close vaults that they have deposited using IDL
* Any user can "deposit" in anyone vault through I direct transfer to the vault.
* Withdrawing will trigger a call to `initialize` method from [this address](https://explorer.solana.com/address/TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM/idl?cluster=devnet)
* A PDA is initialized for vault state using keys (`state`, user public key, state_bump). This PDA
* A PDA is considering for holding funds, using keys (`vault`, vault state PDA, and vault_bump).


Vault state storage variables:
* vault_bump (`u8`): ensures PDA is not a point in ed25519
* state_bump (`u8`): ensures PDA is not a point in ed25519

Vault entry points:
* Initialize: Initialize vault state
* Deposit: Allows vault initializer to dpeosit funds into vault
* Close: Allows vault initializer to withdraw all funds from the vault
* Withdraw: Allos vault initializer to get specified funds from the vault, and also intialize [`Q3 Pre Reqs Rs`](https://explorer.solana.com/address/TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM/idl?cluster=devnet) with a string that is meant to specify a github user

# POC
https://explorer.solana.com/tx/KxTEP1wEVgqKi1HDLnYkddbEnsmLzJoys9sFfyJ92cgcJzfMSHZywbvQkvwRU26CLVJKUsGbfVXthjoJ9MCQLca?cluster=devnet