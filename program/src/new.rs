use std::mem::size_of;

use forge_api::{
	consts::*,
	instruction::NewV1Args,
	loaders::{load_collection_authority, load_mint, load_program, load_signer, load_uninitialized_pda},
	state::Config
};
use solana_program::{
  account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError
};
use mpl_core::{
  instructions::CreateCollectionV2CpiBuilder,
  types::{Attribute, Attributes, Creator, Plugin, PluginAuthority, PluginAuthorityPair, Royalties, RuleSet}
};

use crate::utils::{create_pda, AccountDeserialize, Discriminator};

pub fn process_new<'a, 'info>(
  accounts: &'a [AccountInfo<'info>],
  args: NewV1Args,
) -> ProgramResult {
	let (required_accounts, additional_accounts) = accounts.split_at(8);
	let [signer, collection_info, collection_authority, config_info, mpl_core_program, token_program, associated_token_program, system_program] = required_accounts
	else {
		return Err(ProgramError::NotEnoughAccountKeys);
	};


	load_signer(signer)?;
	load_collection_authority(
        collection_authority,
        &[COLLECTION_AUTHORITY_SEED],
        args.collection_authority_bump,
        &forge_api::id(),
    )?;
	load_uninitialized_pda(
        config_info,
        &[
            CONFIG_SEED,
            collection_info.key.as_ref(),
        ],
        args.config_bump,
        &forge_api::id(),
    )?;
	load_program(token_program, spl_token::ID)?;
	load_program(associated_token_program, spl_associated_token_account::ID)?;
	load_program(mpl_core_program, mpl_core::ID)?;
	load_program(system_program, solana_program::system_program::ID)?;

	// Check signer.
	if signer.key.ne(&INITIALIZER_ADDRESS) {
		return Err(ProgramError::MissingRequiredSignature);
	}

	// Initialize config.
	create_pda(
		config_info,
		&forge_api::id(),
		8 + size_of::<Config>(),
		&[CONFIG_SEED, collection_info.key.as_ref(), &[args.config_bump]],
		system_program,
		signer,
	)?;
	let mut config_data = config_info.data.borrow_mut();
	config_data[0] = Config::discriminator() as u8;
	let config: &mut Config = Config::try_from_bytes_mut(&mut config_data)?;
	config.amounts = args.amounts;
	config.ingredients = args.ingredients;

	// Initialize treasury token accounts if required
	for i in 0..config.ingredients.len() {
		let ingredient = config.ingredients[i];

		if ingredient.eq(&solana_program::system_program::ID) {
			continue;
		}

		let mint_info = &additional_accounts[i];
		load_mint(mint_info, ingredient, false)?;
	}

	let collection_authority_seeds = &[b"collection_authority".as_ref(), &[args.collection_authority_bump]];
	
	CreateCollectionV2CpiBuilder::new(mpl_core_program)
		.collection(collection_info)
		.payer(signer)
		.update_authority(Some(collection_authority))
		.name(args.name)
		.uri(args.uri)
		.plugins(vec![
			PluginAuthorityPair {
				plugin: Plugin::Attributes(Attributes {
					attribute_list: vec![
						Attribute {
							key: "multiplier".to_string(),
							value: args.multiplier.to_string(),
						},
						Attribute {
							key: "durability".to_string(),
							value: args.durability.to_string(),
						},
						Attribute {
							key: "rarity".to_string(),
							value: "common".to_string(),
						},
						Attribute {
							key: "resource".to_string(),
							value: "coal".to_string(),
						},
					],
				}),
				authority: Some(PluginAuthority::UpdateAuthority),
			},
			PluginAuthorityPair {
				plugin: Plugin::Royalties(Royalties {
					basis_points: ROYALTIES_BASIS_POINTS,
					creators: vec![
						Creator {
							address: ROYALTY_CREATOR_ADDRESS,
							percentage: 100,
						},
					],
					rule_set: RuleSet::None,
				}),
				authority: Some(PluginAuthority::UpdateAuthority),
			},
		])
		.system_program(system_program)
		.invoke_signed(&[collection_authority_seeds])?;

  	Ok(())
}
