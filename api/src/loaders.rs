
use forge_utils::AccountDeserialize;
use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
    sysvar,
    msg,
};
use spl_token::state::Mint;
use mpl_core::{Asset, types::UpdateAuthority};

use crate::{
    consts::*, state::{Config, Enhancer, Treasury}, utils::Discriminator
};

/// Errors if:
/// - Account is not a signer.
pub fn load_signer<'a, 'info>(info: &'a AccountInfo<'info>) -> Result<(), ProgramError> {
    if !info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not SPL token program.
/// - Address does not match the expected mint address.
/// - Data is empty.
/// - Data cannot deserialize into a mint account.
/// - Expected to be writable, but is not.
pub fn load_mint<'a, 'info>(
    info: &'a AccountInfo<'info>,
    address: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&spl_token::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.key.ne(&address) {
        return Err(ProgramError::InvalidSeeds);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    Mint::unpack(&info.data.borrow())?;

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not SPL token program.
/// - Data is empty.
/// - Data cannot deserialize into a token account.
/// - Token account owner does not match the expected owner address.
/// - Token account mint does not match the expected mint address.
/// - Expected to be writable, but is not.
pub fn load_token_account<'a, 'info>(
    info: &'a AccountInfo<'info>,
    owner: Option<&Pubkey>,
    mint: &Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&spl_token::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let account_data = info.data.borrow();
    let account = spl_token::state::Account::unpack(&account_data)?;

    if account.mint.ne(&mint) {
        msg!("Invalid mint: {:?} == {:?}", account.mint, mint);
        return Err(ProgramError::InvalidAccountData);
    }

    if let Some(owner) = owner {
        if account.owner.ne(owner) {
            msg!("Invalid owner: {:?} == {:?}", account.owner, owner);
            return Err(ProgramError::InvalidAccountData);
        }
    }

    if is_writable && !info.is_writable {
        msg!("Invalid writable: {:?} == {:?}", info.is_writable, is_writable);
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Address does not match the expected treasury tokens address.
/// - Cannot load as a token accoun
pub fn load_treasury_token_account<'a, 'info>(
    info: &'a AccountInfo<'info>,
    mint: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    let treasury_tokens = spl_associated_token_account::get_associated_token_address(&TREASURY_ADDRESS, &mint);
    
    if info.key.ne(&treasury_tokens) {
        return Err(ProgramError::InvalidSeeds);
    }

    load_token_account(info, Some(&TREASURY_ADDRESS), &mint, is_writable)
}

pub fn load_collection_authority<'a, 'info>(
    info: &'a AccountInfo<'info>,
    seeds: &[&[u8]],
    bump: u8,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    let pda = Pubkey::find_program_address(seeds, program_id);

    if info.key.ne(&pda.0) {
        return Err(ProgramError::InvalidSeeds);
    }

    if bump.ne(&pda.1) {
        return Err(ProgramError::InvalidSeeds);
    }

    Ok(())
}

/// Errors if:
/// - Address does not match PDA derived from provided seeds.
/// - Cannot load as an uninitialized account.
pub fn load_uninitialized_pda<'a, 'info>(
    info: &'a AccountInfo<'info>,
    seeds: &[&[u8]],
    bump: u8,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    let pda = Pubkey::find_program_address(seeds, program_id);

    if info.key.ne(&pda.0) {
        return Err(ProgramError::InvalidSeeds);
    }

    if bump.ne(&pda.1) {
        return Err(ProgramError::InvalidSeeds);
    }

    load_system_account(info, true)
}

/// Errors if:
/// - Owner is not the system program.
/// - Data is not empty.
/// - Account is not writable.
pub fn load_system_account<'a, 'info>(
    info: &'a AccountInfo<'info>,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&system_program::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !info.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Address does not match the expected value.
/// - Account is not executable.
pub fn load_program<'a, 'info>(
    info: &'a AccountInfo<'info>,
    key: Pubkey,
) -> Result<(), ProgramError> {
    if info.key.ne(&key) {
        return Err(ProgramError::IncorrectProgramId);
    }

    if !info.executable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not Ore program.
/// - Address does not match the expected address.
/// - Data is empty.
/// - Data cannot deserialize into a coal config account.
/// - Expected to be writable, but is not.
pub fn load_config<'a, 'info>(
    info: &'a AccountInfo<'info>,
    collection: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    msg!("config_info: {:?}", info.key);
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let pda = Pubkey::find_program_address(&[CONFIG_SEED, collection.as_ref()], &crate::id()).0;
    if info.key.ne(&pda) {
        return Err(ProgramError::InvalidSeeds);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    if info.data.borrow()[0].ne(&(Config::discriminator() as u8)) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not Ore program.
/// - Address does not match the expected address.
/// - Data is empty.
/// - Data cannot deserialize into a treasury account.
/// - Expected to be writable, but is not.
pub fn load_treasury<'a, 'info>(
    info: &'a AccountInfo<'info>,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.key.ne(&TREASURY_ADDRESS) {
        return Err(ProgramError::InvalidSeeds);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    if info.data.borrow()[0].ne(&(Treasury::discriminator() as u8)) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not Coal program.
/// - Data is empty.
/// - Data cannot deserialize into a proof account.
/// - Proof authority does not match the expected address.
/// - Expected to be writable, but is not.
pub fn load_enhance<'a, 'info>(
    info: &'a AccountInfo<'info>,
    authority: &Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let data = info.data.borrow();
    let enhancer = Enhancer::try_from_bytes(&data)?;

    if enhancer.authority.ne(&authority) {
        return Err(ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Data is empty.
/// - Update authority is not the forge pickaxe collection.
/// - Attributes plugin is not present.
/// - Durability attribute is not present.
/// - Multiplier attribute is not present.
pub fn load_asset<'a, 'info>(
    info: &'a AccountInfo<'info>,
) -> Result<(f64, u64, String), ProgramError> {
    if info.owner.ne(&mpl_core::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let asset = Asset::from_bytes(&info.data.borrow()).unwrap();

    match asset.base.update_authority {
        UpdateAuthority::Collection(address) => {
            if address.ne(&COLLECTION) {
                msg!("Invalid collection: {:?} == {:?}", address, COLLECTION);
                return Err(ProgramError::InvalidAccountData);
            }
        }
        _ => return Err(ProgramError::InvalidAccountData),
    }

    if asset.plugin_list.attributes.is_none() {
        return Err(ProgramError::InvalidAccountData);
    }

	let attributes_plugin = asset.plugin_list.attributes.unwrap();
	let durability_attr = attributes_plugin.attributes.attribute_list.iter().find(|attr| attr.key == "durability");
	let multiplier_attr = attributes_plugin.attributes.attribute_list.iter().find(|attr| attr.key == "multiplier");
    let resource_attr = attributes_plugin.attributes.attribute_list.iter().find(|attr| attr.key == "resource");
    let durability = durability_attr.unwrap().value.parse::<f64>().unwrap();
    let multiplier = multiplier_attr.unwrap().value.parse::<u64>().unwrap();
    let resource = resource_attr.unwrap().value.clone();
    
    Ok((durability, multiplier, resource))
}

/// Errors if:
/// - Owner is not the sysvar address.
/// - Account cannot load with the expected address.
pub fn load_sysvar<'a, 'info>(
    info: &'a AccountInfo<'info>,
    key: Pubkey,
) -> Result<(), ProgramError> {
    if info.owner.ne(&sysvar::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    load_account(info, key, false)
}

/// Errors if:
/// - Address does not match the expected value.
/// - Expected to be writable, but is not.
pub fn load_account<'a, 'info>(
    info: &'a AccountInfo<'info>,
    key: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.key.ne(&key) {
        return Err(ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

