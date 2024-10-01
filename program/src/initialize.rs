use std::mem::size_of;

use forge_api::{consts::{INITIALIZER_ADDRESS, TREASURY}, instruction::InitializeArgs, loaders::load_signer, state::Treasury};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
};

use crate::utils::{create_pda, Discriminator};

/// Initialize the forge treasury.
pub fn process_initialize<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    args: InitializeArgs,
) -> ProgramResult {
    // Load accounts.
    let [signer, treasury_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    load_signer(signer)?;

    // Check signer.
    if signer.key.ne(&INITIALIZER_ADDRESS) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Initialize treasury.
    create_pda(
        treasury_info,
        &forge_api::id(),
        8 + size_of::<Treasury>(),
        &[TREASURY, &[args.treasury_bump]],
        system_program,
        signer,
    )?;
    let mut treasury_data = treasury_info.data.borrow_mut();
    treasury_data[0] = Treasury::discriminator() as u8;

    Ok(())
}
