//! serum-safe defines the interface for the serum safe program.

#![cfg_attr(feature = "strict", deny(warnings))]

use serum_common::pack::*;
use solana_client_gen::prelude::*;

pub mod accounts;
pub mod error;

#[cfg_attr(feature = "client", solana_client_gen)]
pub mod instruction {
    use super::*;
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum RegistryInstruction {
        /// Initializes the registry instance for use. Anyone can invoke this
        /// instruction so it should be run in the same transaction as the
        /// create_account instruction for the Registry instance.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]` Registry to initialize.
        /// 1. `[]`         Mint of the token to pay rewards with (SRM).
        /// 2. `[]`         Mint of the "Mega" token to pay rewards with (MSRM).
        /// 3. `[]`         Rent sysvar.
        Initialize {
            /// The priviledged account.
            authority: Pubkey,
            /// The nonce used to create the Registry's program-derived address,
            /// which owns it's token vault.
            nonce: u8,
            /// Nonce for the program's vault holding the Mega token (MSRM).
            mega_nonce: u8,
        },
        /// Donates funds into the registry for reward distribution. Anyone
        /// can invoke this instruction.
        ///
        /// 0. `[signer]`   Owner of the account sending the funds.
        /// 1. `[writable]` Account from which to send the funds.
        /// 2. `[writable]` Program controlled token vault to transfer funds
        ///                 into.
        /// 3. `[]`         Registry instance, holding the nonce to calculate
        ///                 the program-derived-address.
        /// 4. `[]`         SPL token program.
        Donate {
            /// The amount to deposit.
            amount: u64,
            /// True if the donation is denominated in the mega token.
            is_mega: bool,
        },
        /// CreateEntity initializes the new "node" with the Registry, allowing
        /// addresses to stake with it and collect rewards.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]` Entity account.
        /// 1. `[signer]`   Leader of the node.
        /// 2. `[]`         Rent sysvar.
        CreateEntity {
            capabilities: u32,
            stake_kind: accounts::StakeKind,
        },
        /// UpdateEntity updates the capabilities of the node entity.
        ///
        /// Accounts:
        ///
        /// 0. `[writable]` Entity account.
        /// 1. `[signer]`   Leader of the node.
        /// 2. `[]`         Rent sysvar.
        UpdateEntity { capabilities: u32 },
        /// RegisterCapability adds a node capability for reward collection,
        /// or overwrites an existing capability (e.g., on program upgrade).
        ///
        /// Accounts:
        ///
        /// 0. `[signer]`   Registry authority.
        /// 1. `[writable]` Registry instance.
        RegisterCapability {
            /// The identifier to assign this capability.
            capability_id: u8,
            /// The external address used to calculate rewards for this
            /// capability.
            capability_program: Pubkey,
            /// The percent of the registry's reward distribution assigned to
            /// this capability.
            ///
            /// After registration, all other capabilities' weights will be
            /// normalized to account for this new capability.
            capability_weight: f32,
        },
        /// Assigns the reward weights of all capabilities explicitly.
        ///
        /// Accounts:
        ///
        /// 0. `[signer]`   Entity authority.
        /// 1. `[writable]` Registry instance.
        SetCapabilityWeights { capability_weights: [f32; 32] },
        /// Stake deposits funds into a registered node entity pool,
        /// initializing the given beneficiary as a staker, if it's not already
        /// initialized.
        ///
        /// Accounts:
        ///
        /// 0. `[signed]`   Owner of the account to send funds from.
        /// 1. `[writable]` Token account containing the funds.
        /// 2. `[writable]` Stake account.
        /// 3. `[]`         Registry instance.
        /// 4. `[writable]` Program controlled token vault to transfer funds
        ///                 into.
        /// 5. `[writable]` Entity account to stake with.
        /// 6. `[]`         SPL token program.
        Stake {
            /// The amount to stake.
            amount: u64,
            /// The key to associate wtih this deposit. Collecting rewards
            /// will require this key to sign.
            beneficiary: Pubkey,
            /// True iff the token being transferred is the mega token.
            is_mega: bool,
        },
        /// StakeLocked is the same as the `Stake` instruction, but using
        /// the locked token minted from the Serum Safe.
        ///
        /// Accounts:
        ///
        ///
        StakeLocked, // todo
        /// Deposits more funds into a given staking account.
        ///
        /// Accounts:
        ///
        /// 0. `[signed]`   Owner of the account to send funds from.
        /// 1. `[writable]` Token account containing the funds to send from.
        /// 2. `[writable]` Stake account to add to.
        /// 3. `[]`         Registry instance.
        /// 4. `[writable]` Program controlled token vault to transfer funds
        ///                 into.
        /// 5. `[writable]` Entity account the stake is associated with.
        /// 6. `[]`         SPL token program.
        AddStake { amount: u64 },
        /// Collect rewards retrieves rewards earned from node duties.
        ///
        /// Accounts:
        ///
        /// 0. `[signed]`   Beneficiary of the Stake account to collect rewards.
        /// 1. `[]`         Token account to send rewards to.
        /// 2. `[]`         Stake account from which to get rewards.
        /// 3. `[]`         Entity the stake account is associated with.
        /// 4. `[writable]` Registry token vault issuing the rewards.
        /// 5. `[]`         SPL token program (SRM).
        CollectRewards,
        /// Initiates a stake withdrawal. Funds are locked up until the
        /// withdrawl timelock passes.
        ///
        /// Accounts:
        ///
        /// 0  `[signed]`   Benficiary of the Stake account.
        /// 1. `[writable]` The Stake account to withdraw from.
        /// 2. `[writable]` Entity the Stake is associated with.
        InitiateStakeWithdrawal { amount: u64, mega_amount: u64 },
        /// Completes the initiated withdrawal.
        ///
        /// Accounts:
        ///
        /// 0. `[signed]`   Beneficiary of the Stake account.
        /// 1. `[writable]` Stake account to withdraw from.
        /// 3. `[writable]` Entity the Stake is associated with.
        /// 4. `[writable]` Program controlled token vault to transfer funds
        ///                 into.
        /// 5. `[]`         Registry instance.
        /// 6. `[]`         SPL token program (SRM).
        /// 7. `[]`         SPL mega token program (MSRM).
        /// 8. `[writable]` The token account to send funds to.
        /// 9. `[writable]` The mega token account to send funds to.
        CompleteStakeWithdrawal {
            // True if we want to withdraw the normal token out.
            is_token: bool,
            // True if we want to wtihdraw the mega token out.
            is_mega: bool,
        },
    }
}

serum_common::packable!(crate::instruction::RegistryInstruction);
