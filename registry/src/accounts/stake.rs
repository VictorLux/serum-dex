use serum_common::pack::*;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

/// Stake represents an individuals staking deposit with an entity in the
/// registry.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Stake {
    /// Set when the program accepts the staking deposit.
    pub initialized: bool,
    /// The authority that can collect rewards from this stake.
    pub beneficiary: Pubkey,
    /// The entity this stake is associated with.
    pub entity_id: Pubkey,
    /// The amount of funds staked.
    pub amount: u64,
    /// The amount of mega funds staked.
    pub mega_amount: u64,
}

serum_common::packable!(Stake);
