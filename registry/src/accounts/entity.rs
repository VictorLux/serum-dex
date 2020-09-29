use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};
use serum_common::pack::*;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

/// Entity is the account representing a single "node" that addresses can
/// stake with.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    /// Leader of the entity, i.e., the one responsible for fulfilling node
    /// duties.
    pub leader: Pubkey,
    /// Set when this entity is registered with the program.
    pub initialized: bool,
    /// The amount of the token pooled in this entity.
    pub amount: u64,
    /// The amount of the mega token pooled in this entity.
    pub mega_amount: u64,
    /// Bitmap representing this entity's capabilities .
    // TODO: increase this and then convert the registry to manual serialization.
    pub capabilities: u32,
    /// The type of stake backing this entity (determines voting rights)
    /// of the stsakers.
    pub stake_kind: StakeKind,
}

#[derive(Debug, IntoPrimitive, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
pub enum StakeKind {
    Voting,
    Delegated,
}

serum_common::packable!(Entity);
