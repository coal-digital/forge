use std::mem::size_of;

use forge_api::{
    consts::{CHROMIUM_MINT_ADDRESS, ENHANCER_SEED, ENHANCER_TARGET_SLOT}, 
    instruction::InitializeEnhanceArgs, 
    loaders::{load_asset, load_mint, load_signer, load_sysvar, load_token_account, load_uninitialized_pda}, state::Enhancer 
};
use forge_utils::spl::burn;
use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    entrypoint::ProgramResult,
    keccak::hashv,
    program_error::ProgramError,
    slot_hashes::SlotHash, 
    sysvar::{self, Sysvar}
};

use crate::utils::{create_pda, AccountDeserialize, Discriminator};


pub fn process_initialize_enhance(accounts: &[AccountInfo], args: InitializeEnhanceArgs) -> ProgramResult {
    // Load accounts.
    let [signer, asset_info, enhancer_info, chromium_mint_info, chromium_tokens_info, slot_hashes_sysvar, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_signer(signer)?;
    load_mint(chromium_mint_info, CHROMIUM_MINT_ADDRESS, true)?;
    load_token_account(chromium_tokens_info, Some(signer.key), &chromium_mint_info.key, true)?;
    load_uninitialized_pda(
        enhancer_info,
        &[ENHANCER_SEED, signer.key.as_ref(), asset_info.key.as_ref()],
        args.enhancer_bump,
        &forge_api::id(),
    )?;
    load_sysvar(slot_hashes_sysvar, sysvar::slot_hashes::id())?;
    
    let (durability, _multiplier, _resource) = load_asset(asset_info)?;

    // Initialize reprocessor.
    create_pda(
        enhancer_info,
        &forge_api::id(),
        8 + size_of::<Enhancer>(),
        &[ENHANCER_SEED, signer.key.as_ref(), &[args.enhancer_bump]],
        system_program,
        signer,
    )?;

    let mut enhancer_data = enhancer_info.data.borrow_mut();
    enhancer_data[0] = Enhancer::discriminator() as u8;
    let enhancer = Enhancer::try_from_bytes_mut(&mut enhancer_data)?;
    enhancer.authority = *signer.key;
    
    let slot = Clock::get()?.slot;
    enhancer.slot = slot + ENHANCER_TARGET_SLOT;
    enhancer.hash = hashv(&[
        &slot_hashes_sysvar.data.borrow()[0..size_of::<SlotHash>()],
    ])
    .0;

    // Burn ingredient tokens
    burn(
        chromium_tokens_info, 
        chromium_mint_info,
        signer,
        chromium_tokens_info,
        durability as u64
    )?;

    Ok(())
}