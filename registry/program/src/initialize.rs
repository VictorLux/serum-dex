use serum_common::pack::Pack;
use serum_registry::accounts::Registry;
use serum_registry::error::{RegistryError, RegistryErrorCode};
use solana_sdk::account_info::{next_account_info, AccountInfo};
use solana_sdk::info;
use solana_sdk::pubkey::Pubkey;

pub fn handler<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    authority: Pubkey,
    nonce: u8,
    mega_nonce: u8,
) -> Result<(), RegistryError> {
    info!("handler: registry");

    let acc_infos = &mut accounts.iter();

    let registry_acc_info = next_account_info(acc_infos)?;
    let mint_acc_info = next_account_info(acc_infos)?;
    let mega_mint_acc_info = next_account_info(acc_infos)?;
    let rent_acc_info = next_account_info(acc_infos)?;

    access_control(AccessControlRequest {
        registry_acc_info,
        mint_acc_info,
        mega_mint_acc_info,
        rent_acc_info,
        nonce,
        mega_nonce,
    })?;

    Registry::unpack_mut(
        &mut registry_acc_info.try_borrow_mut_data()?,
        &mut |registry: &mut Registry| {
            state_transition(StateTransitionRequest {
                mint: mint_acc_info.key,
                mega_mint: mega_mint_acc_info.key,
                registry,
                authority,
                nonce,
                mega_nonce,
            })
            .map_err(Into::into)
        },
    )?;

    Ok(())
}

fn access_control<'a>(req: AccessControlRequest<'a>) -> Result<(), RegistryError> {
    info!("access-control: registry");

    let AccessControlRequest {
        registry_acc_info,
        mint_acc_info,
        mega_mint_acc_info,
        rent_acc_info,
        nonce,
        mega_nonce,
    } = req;

    // todo

    info!("access-control: success");

    Ok(())
}

fn state_transition<'a>(req: StateTransitionRequest<'a>) -> Result<(), RegistryError> {
    info!("state-transition: initialize");

    let StateTransitionRequest {
        registry,
        authority,
        mint,
        mega_mint,
        nonce,
        mega_nonce,
    } = req;

    registry.initialized = true;
    registry.mint = *mint;
    registry.mega_mint = *mega_mint;
    registry.nonce = nonce;
    registry.mega_nonce = mega_nonce;
    registry.capabilities = [Pubkey::new_from_array([0; 32]); 32];
    registry.capability_weights = [0.0; 32];
    registry.authority = authority;

    info!("state-transition: success");

    Ok(())
}

struct AccessControlRequest<'a> {
    registry_acc_info: &'a AccountInfo<'a>,
    mint_acc_info: &'a AccountInfo<'a>,
    mega_mint_acc_info: &'a AccountInfo<'a>,
    rent_acc_info: &'a AccountInfo<'a>,
    nonce: u8,
    mega_nonce: u8,
}

struct StateTransitionRequest<'a> {
    registry: &'a mut Registry,
    authority: Pubkey,
    mint: &'a Pubkey,
    mega_mint: &'a Pubkey,
    nonce: u8,
    mega_nonce: u8,
}
