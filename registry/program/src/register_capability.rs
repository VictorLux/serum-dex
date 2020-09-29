use serum_common::pack::Pack;
use serum_registry::accounts::Registry;
use serum_registry::error::{RegistryError, RegistryErrorCode};
use solana_sdk::account_info::{next_account_info, AccountInfo};
use solana_sdk::info;
use solana_sdk::pubkey::Pubkey;

pub fn handler<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    capability_id: u8,
    capability_program: Pubkey,
    capability_weight: f32,
) -> Result<(), RegistryError> {
    info!("handler: register_capability");

    let acc_infos = &mut accounts.iter();

    let registry_authority_acc_info = next_account_info(acc_infos)?;
    let registry_acc_info = next_account_info(acc_infos)?;

    access_control(AccessControlRequest {
        registry_authority_acc_info,
        registry_acc_info,
        capability_id,
        capability_weight,
    })?;

    Registry::unpack_mut(
        &mut registry_acc_info.try_borrow_mut_data()?,
        &mut |registry: &mut Registry| {
            state_transition(StateTransitionRequest {
                registry,
                capability_id,
                capability_program,
                capability_weight,
            })
            .map_err(Into::into)
        },
    )?;

    Ok(())
}

fn access_control(req: AccessControlRequest) -> Result<(), RegistryError> {
    info!("access-control: register_capability");

    let AccessControlRequest {
        registry_authority_acc_info,
        registry_acc_info,
        capability_id,
        capability_weight,
    } = req;

    // todo

    info!("access-control: success");

    Ok(())
}

fn state_transition(req: StateTransitionRequest) -> Result<(), RegistryError> {
    info!("state-transition: register_capability");

    let StateTransitionRequest {
        mut registry,
        capability_id,
        capability_program,
        capability_weight,
    } = req;

    normalize_weights(&mut registry, capability_weight);
    registry.capabilities[capability_id as usize] = capability_program;
    registry.capability_weights[capability_id as usize] = capability_weight;

    info!("state-transition: success");

    Ok(())
}

fn normalize_weights(registry: &mut Registry, new_weight: f32) {
    let current_total_weight: f32 = registry.capability_weights.iter().sum();
    let new_total_weight = current_total_weight + new_weight;
    for weight in registry.capability_weights.iter_mut() {
        *weight = *weight / new_total_weight;
    }
}

struct AccessControlRequest<'a> {
    registry_authority_acc_info: &'a AccountInfo<'a>,
    registry_acc_info: &'a AccountInfo<'a>,
    capability_id: u8,
    capability_weight: f32,
}

struct StateTransitionRequest<'a> {
    registry: &'a mut Registry,
    capability_id: u8,
    capability_program: Pubkey,
    capability_weight: f32,
}
