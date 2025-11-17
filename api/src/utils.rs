use mpl_core::{
    instructions::UpdatePluginV1CpiBuilder,
    types::{Attribute, Attributes, Plugin, UpdateAuthority},
    Asset,
};
use solana_program::msg;
use steel::*;

use crate::consts::TOKEN_DECIMALS;

/// Errors if:
/// - Data is empty.
/// - Update authority is not the expected collection.
/// - Attributes plugin is not present.
/// - Durability attribute is not present.
/// - Multiplier attribute is not present.
pub fn load_asset<'a, 'info>(
    info: &'a AccountInfo<'info>,
    expected_collection: Pubkey,
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
            if address.ne(&expected_collection) {
                msg!(
                    "Invalid collection: {:?} != {:?}",
                    address,
                    expected_collection
                );
                return Err(ProgramError::InvalidAccountData);
            }
        }
        _ => return Err(ProgramError::InvalidAccountData),
    }

    if asset.plugin_list.attributes.is_none() {
        return Err(ProgramError::InvalidAccountData);
    }

    let attributes_plugin = asset.plugin_list.attributes.unwrap();
    let durability_attr = attributes_plugin
        .attributes
        .attribute_list
        .iter()
        .find(|attr| attr.key == "durability");
    let multiplier_attr = attributes_plugin
        .attributes
        .attribute_list
        .iter()
        .find(|attr| attr.key == "multiplier");
    let resource_attr = attributes_plugin
        .attributes
        .attribute_list
        .iter()
        .find(|attr| attr.key == "resource");
    let durability = durability_attr.unwrap().value.parse::<f64>().unwrap();
    let multiplier = multiplier_attr.unwrap().value.parse::<u64>().unwrap();
    let resource = resource_attr.unwrap().value.clone();

    Ok((durability, multiplier, resource))
}

pub fn amount_u64_to_f64(amount: u64) -> f64 {
    (amount as f64) / 10f64.powf(TOKEN_DECIMALS as f64)
}

pub fn amount_f64_to_u64(amount: f64) -> u64 {
    (amount * 10f64.powf(TOKEN_DECIMALS as f64)) as u64
}
