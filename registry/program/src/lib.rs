//! Program entrypoint.

#![cfg_attr(feature = "strict", deny(warnings))]

use serum_common::pack::Pack;
use serum_registry::error::{RegistryError, RegistryErrorCode};
use serum_registry::instruction::RegistryInstruction;
use solana_sdk::account_info::AccountInfo;
use solana_sdk::entrypoint::ProgramResult;
use solana_sdk::info;
use solana_sdk::pubkey::Pubkey;

mod collect_rewards;
mod complete_stake_withdrawal;
mod create_entity;
mod donate;
mod initialize;
mod initiate_stake_withdrawal;
mod register_capability;
mod set_capability_weights;
mod stake;
mod update_entity;

solana_sdk::entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    info!("process-instruction");

    let instruction: RegistryInstruction = RegistryInstruction::unpack(instruction_data)
        .map_err(|_| RegistryError::ErrorCode(RegistryErrorCode::WrongSerialization))?;

    let result = match instruction {
        RegistryInstruction::Initialize {
            authority,
            nonce,
            mega_nonce,
        } => initialize::handler(program_id, accounts, authority, nonce, mega_nonce),
        RegistryInstruction::Donate { amount, is_mega } => {
            donate::handler(program_id, accounts, amount, is_mega)
        }
        RegistryInstruction::CreateEntity {
            capabilities,
            stake_kind,
        } => create_entity::handler(program_id, accounts, capabilities, stake_kind),
        RegistryInstruction::UpdateEntity {
            capabilities,
            // todo: do we want to all the leader to update voting type?
        } => update_entity::handler(program_id, accounts, capabilities),
        RegistryInstruction::RegisterCapability {
            capability_id,
            capability_program,
            capability_weight,
        } => register_capability::handler(
            program_id,
            accounts,
            capability_id,
            capability_program,
            capability_weight,
        ),
        RegistryInstruction::SetCapabilityWeights { capability_weights } => {
            set_capability_weights::handler(program_id, accounts, capability_weights)
        }
        RegistryInstruction::Stake {
            amount,
            beneficiary,
            is_mega,
        } => stake::handler(program_id, accounts, amount, beneficiary, is_mega),
        RegistryInstruction::AddStake { amount } => {
            // todo
            Ok(())
        }
        RegistryInstruction::StakeLocked => {
            // todo
            Ok(())
        }
        RegistryInstruction::CollectRewards => collect_rewards::handler(program_id, accounts),
        RegistryInstruction::InitiateStakeWithdrawal {
            amount,
            mega_amount,
        } => initiate_stake_withdrawal::handler(program_id, accounts),
        RegistryInstruction::CompleteStakeWithdrawal { is_token, is_mega } => {
            complete_stake_withdrawal::handler(program_id, accounts)
        }
    };

    result?;

    info!("process-instruction success");

    Ok(())
}
