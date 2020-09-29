use serum_common::pack::Pack;
use serum_registry::accounts::Registry;
use serum_registry::error::{RegistryError, RegistryErrorCode};
use solana_sdk::account_info::{next_account_info, AccountInfo};
use solana_sdk::info;
use solana_sdk::pubkey::Pubkey;

pub fn handler<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    capability_weights: [f32; 32],
) -> Result<(), RegistryError> {
    info!("handler: set_capability_weights");

    let acc_infos = &mut accounts.iter();

    let registry_authority_acc_info = next_account_info(acc_infos)?;
    let registry_acc_info = next_account_info(acc_infos)?;

    access_control(AccessControlRequest {
        registry_authority_acc_info,
        registry_acc_info,
        capability_weights,
    })?;

    Registry::unpack_mut(
        &mut registry_acc_info.try_borrow_mut_data()?,
        &mut |registry: &mut Registry| {
            state_transition(StateTransitionRequest {
                registry,
                capability_weights,
            })
            .map_err(Into::into)
        },
    )
    .map_err(Into::into)
}

fn access_control(req: AccessControlRequest) -> Result<(), RegistryError> {
    info!("access-control: set_capability_weights");

    let AccessControlRequest {
        registry_authority_acc_info,
        registry_acc_info,
        capability_weights,
    } = req;

    // todo

    info!("access-control: success");

    Ok(())
}

fn state_transition(req: StateTransitionRequest) -> Result<(), RegistryError> {
    info!("state-transition: set_capability_weights");

    let StateTransitionRequest {
        registry,
        capability_weights,
    } = req;

    registry.capability_weights = capability_weights;

    info!("state-transition: success");

    Ok(())
}

struct AccessControlRequest<'a> {
    registry_authority_acc_info: &'a AccountInfo<'a>,
    registry_acc_info: &'a AccountInfo<'a>,
    capability_weights: [f32; 32],
}

struct StateTransitionRequest<'a> {
    registry: &'a mut Registry,
    capability_weights: [f32; 32],
}
