use std::mem::size_of;

use forge_api::{
    consts::*, error::ForgeError, instruction::EnhanceArgs, loaders::*, state::Enhancer
};
use mpl_core::{
    instructions::{BurnV1CpiBuilder,CreateV2CpiBuilder},
    types::{Attribute, Attributes, Plugin, PluginAuthority, PluginAuthorityPair},
    Asset,
    Collection
};
use solana_program::{
    msg,
    account_info::AccountInfo,
    clock::Clock, 
    entrypoint::ProgramResult, 
    program_error::ProgramError, 
    slot_hashes::SlotHash, 
    sysvar::{self, Sysvar},
    keccak::hashv
};

use crate::utils::AccountDeserialize;

pub fn process_enhance(accounts: &[AccountInfo], args: EnhanceArgs) -> ProgramResult {
    // Load accounts.
    let [signer, new_mint_info, asset_info, collection_info, collection_authority, enhancer_info, mpl_core_program, system_program, slot_hashes_sysvar] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_signer(signer)?;
    load_enhance(enhancer_info, signer.key, asset_info.key, true)?;
    load_sysvar(slot_hashes_sysvar, sysvar::slot_hashes::id())?;
    
    let mut enhancer_data = enhancer_info.data.borrow_mut();
    let enhancer = Enhancer::try_from_bytes_mut(&mut enhancer_data)?;
    
    // Target slot is 20 slots ahead of the starting slot
    let target_slot = enhancer.slot;
    let current_slot = Clock::get()?.slot;
    
    // Check if the current slot is less than the target slot
    if current_slot.le(&target_slot) {
        return Err(ForgeError::SlotTooEarly.into());
    }

    // Calculate the final hash
    let final_hash = hashv(&[
        &enhancer.hash,
        &slot_hashes_sysvar.data.borrow()[0..size_of::<SlotHash>()],
    ])
    .0;

    drop(enhancer_data);

    // Derive a number between 320 and 600 from the enhancement
    let mut pseudo_random_number = derive_number_from_hash(&final_hash, ENHANCE_MIN_MULTIPLIER, ENHANCE_MAX_MULTIPLIER);

    // Calculate the liveness penalty
    let s_tolerance = target_slot.saturating_add(ENHANCE_SLOT_BUFFER);
    
    msg!("Current slot: {}", current_slot);
    msg!("Target slot: {}", target_slot);
    
    if current_slot.gt(&s_tolerance) {
        // Halve the reward for every slot late.
        let halvings = current_slot.saturating_sub(s_tolerance) as u64;
        msg!("Halvings: {}", halvings);
        if halvings.gt(&0) {
            pseudo_random_number = pseudo_random_number.saturating_div(2u64.saturating_pow(halvings as u32)).max(ENHANCE_MIN_MULTIPLIER);
        }
    }

    msg!("Derived number: {}", pseudo_random_number);

	// Update attributes
	let asset = Asset::from_bytes(&asset_info.data.borrow()).unwrap();
	let attributes_plugin = asset.plugin_list.attributes.unwrap();
	let resource = attributes_plugin.attributes.attribute_list.iter().find(|attr| attr.key == "resource").unwrap().value.clone();
    
    let mut updated_attributes = vec![
		Attribute {
			key: "multiplier".to_string(),
			value: pseudo_random_number.to_string()
		},
        Attribute {
			key: "rarity".to_string(),
			value: "uncommon".to_string()
		},
	];

	attributes_plugin.attributes.attribute_list.iter().for_each(|attr| {
		if attr.key != "multiplier" && attr.key != "rarity" {
			updated_attributes.push(Attribute {
				key: attr.key.clone(),
				value: attr.value.clone(),
			});
		}
	});

    let name = match resource.as_str() {
        "wood" => "Enhanced Woodcutter's Axe".to_string(),
        _ => "Enhanced Miner's Pickaxe".to_string(),
    };

    let uri = match resource.as_str() {
		"coal" => "https://minechain.gg/metadata.pickaxe.uncommon.json".to_string(),
		"wood" => "https://minechain.gg/metadata.axe.uncommon.json".to_string(),
		_ => {
			return Err(ForgeError::InvalidResource.into());
		}
	};

    let collection: Box<Collection> = {
		let collection_data = collection_info.data.borrow();
		Collection::from_bytes(&collection_data).unwrap()
	};
	let royalties_plugin = collection.plugin_list.royalties.unwrap();

	// Update attributes CPI
    let collection_authority_seeds = &[COLLECTION_AUTHORITY_SEED, &[args.collection_authority_bump]];

    CreateV2CpiBuilder::new(mpl_core_program)
      .asset(new_mint_info)
      .collection(Some(&collection_info))
      .payer(signer)
      .owner(Some(signer))
      .name(name)
      .uri(uri)
      .authority(Some(collection_authority))
      .plugins(vec![
          PluginAuthorityPair {
              plugin: Plugin::Attributes(Attributes {
                  attribute_list: updated_attributes
              }),
              authority: Some(PluginAuthority::Address {
                  address: COAL_UPDATE_AUTHORITY,
              }),
          },
          PluginAuthorityPair {
              plugin: Plugin::Royalties(royalties_plugin.royalties),
              authority: Some(PluginAuthority::Address {
                  address: COAL_UPDATE_AUTHORITY,
              }),
          },
      ])
      .system_program(system_program)
      .invoke_signed(&[collection_authority_seeds])?;
    
    // Burn the old asset
    BurnV1CpiBuilder::new(mpl_core_program)
        .asset(asset_info)
        .collection(Some(collection_info))
        .authority(Some(signer))
        .payer(signer)
        .invoke()?;

    // Realloc data to zero.
    enhancer_info.realloc(0, true)?;

    // Send remaining lamports to signer.
    **signer.lamports.borrow_mut() += enhancer_info.lamports();
    **enhancer_info.lamports.borrow_mut() = 0;

    Ok(())

}

// Helper function to derive a number from the entire hash
fn derive_number_from_hash(hash: &[u8; 32], min: u64, max: u64) -> u64 {
    let mut acc = 0u64;
    for chunk in hash.chunks(8) {
        acc = acc.wrapping_add(u64::from_le_bytes(chunk.try_into().unwrap_or([0; 8])));
    }
    min + (acc % (max - min + 1))
}