use serum_common::pack::Pack;
use serum_registry::access_control;
use serum_registry::accounts::Registrar;
use serum_registry::error::RegistryError;
use solana_program::info;
use solana_sdk::account_info::{next_account_info, AccountInfo};
use solana_sdk::pubkey::Pubkey;

pub fn handler(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_authority: Option<Pubkey>,
    withdrawal_timelock: Option<i64>,
    deactivation_timelock: Option<i64>,
    reward_activation_threshold: Option<u64>,
) -> Result<(), RegistryError> {
    info!("handler: initialize");

    let acc_infos = &mut accounts.iter();

    let registrar_acc_info = next_account_info(acc_infos)?;
    let authority_acc_info = next_account_info(acc_infos)?;

    access_control(AccessControlRequest {
        registrar_acc_info,
        authority_acc_info,
        program_id,
    })?;

    Registrar::unpack_mut(
        &mut registrar_acc_info.try_borrow_mut_data()?,
        &mut |registrar: &mut Registrar| {
            state_transition(StateTransitionRequest {
                registrar,
                new_authority,
                withdrawal_timelock,
                deactivation_timelock,
                reward_activation_threshold,
            })
            .map_err(Into::into)
        },
    )?;

    Ok(())
}

fn access_control(req: AccessControlRequest) -> Result<(), RegistryError> {
    info!("access-control: update_registrar");

    let AccessControlRequest {
        registrar_acc_info,
        authority_acc_info,
        program_id,
    } = req;

    // Authorization.
    let _ = access_control::governance(program_id, registrar_acc_info, authority_acc_info)?;

    Ok(())
}

#[inline(always)]
fn state_transition(req: StateTransitionRequest) -> Result<(), RegistryError> {
    info!("state-transition: update_registrar");

    let StateTransitionRequest {
        registrar,
        new_authority,
        withdrawal_timelock,
        deactivation_timelock,
        reward_activation_threshold,
    } = req;

    if let Some(new_authority) = new_authority {
        registrar.authority = new_authority;
    }

    if let Some(withdrawal_timelock) = withdrawal_timelock {
        registrar.withdrawal_timelock = withdrawal_timelock;
    }

    if let Some(deactivation_timelock) = deactivation_timelock {
        registrar.deactivation_timelock = deactivation_timelock;
    }

    if let Some(reward_activation_threshold) = reward_activation_threshold {
        registrar.reward_activation_threshold = reward_activation_threshold;
    }

    Ok(())
}

struct AccessControlRequest<'a, 'b> {
    registrar_acc_info: &'a AccountInfo<'b>,
    authority_acc_info: &'a AccountInfo<'b>,
    program_id: &'a Pubkey,
}

struct StateTransitionRequest<'a> {
    registrar: &'a mut Registrar,
    new_authority: Option<Pubkey>,
    withdrawal_timelock: Option<i64>,
    deactivation_timelock: Option<i64>,
    reward_activation_threshold: Option<u64>,
}
