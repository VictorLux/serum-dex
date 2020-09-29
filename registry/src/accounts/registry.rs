use serum_common::pack::*;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

/// Registry defines the account representing an instance of the program.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Registry {
    /// Set by the program on program initialization.
    pub initialized: bool,
    /// The mint of the SPL token used by the registry (SRM).
    pub mint: Pubkey,
    /// The mint of the mega SPL token used by the registry (MSRM).
    pub mega_mint: Pubkey,
    /// The nonce for the program-derived-address controlling the token
    /// vault.
    pub nonce: u8,
    /// The nonce for the program-derived-address controlling the mega token
    /// vault.
    pub mega_nonce: u8,
    /// Maps capability identifier to the Pubkey address of the program
    /// to calculate rewards for the capability.
    pub capabilities: [Pubkey; 32],
    /// Maps capability identifier to the reward percentage that capability
    /// gets of the total registry reward distribution.
    ///
    /// Invariant: `assert!(capability_weights.iter().sum == 100);`
    pub capability_weights: [f32; 32],
    /// The priviledged account with the ability to register capabilities.
    pub authority: Pubkey,
}

serum_common::packable!(Registry);
