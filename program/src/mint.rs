use forge_api::{
	consts::COAL_UPDATE_AUTHORITY,
	instruction::MintV1Args,
	loaders::{load_config, load_program, load_signer, load_token_account},
	state::Config
};
use forge_utils::{spl::burn, AccountDeserialize};
use solana_program::{
  account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError
};
use mpl_core::{
  Collection,
  instructions::CreateV2CpiBuilder, 
  types::{Plugin, PluginAuthority, PluginAuthorityPair},
};

pub fn process_mint<'a, 'info>(
  accounts: &'a [AccountInfo<'info>],
  args: MintV1Args,
) -> ProgramResult {
	let (required_accounts, remaining_accounts) = accounts.split_at(8);
	let [signer, mint_info, collection_info, collection_authority, config_info, mpl_core_program, token_program, system_program] = required_accounts
	else {
		return Err(ProgramError::NotEnoughAccountKeys);
	};

	load_signer(signer)?;
	load_config(config_info, *collection_info.key, false)?;
	load_program(mpl_core_program, mpl_core::ID)?;
	load_program(token_program, spl_token::ID)?;
	load_program(system_program, solana_program::system_program::ID)?;

	let config_data = config_info.data.borrow();
	let config = Config::try_from_bytes(&config_data).unwrap();

	for i in 0..config.ingredients.len() {
		let ingredient = config.ingredients[i];
		let amount = config.amounts[i];

		if amount == 0 {
			continue;
		}

		let mint_info = &remaining_accounts[i * 2];
		let ingredient_tokens_info = &remaining_accounts[i * 2 + 1];
		
		if ingredient.ne(&mint_info.key) {
			return Err(ProgramError::InvalidAccountData);
		}

		load_token_account(&ingredient_tokens_info, Some(signer.key), &ingredient, true)?;
		
		// Burn ingredient tokens
		burn(
			ingredient_tokens_info, 
			mint_info,
			signer,
			token_program,
			amount
		)?;
	}

	let collection: Box<Collection> = {
		let collection_data = collection_info.data.borrow();
		Collection::from_bytes(&collection_data).unwrap()
	};
	let attributes_plugin = collection.plugin_list.attributes.unwrap();
	let royalties_plugin = collection.plugin_list.royalties.unwrap();
	
	let collection_authority_seeds = &[b"collection_authority".as_ref(), &[args.collection_authority_bump]];

  	CreateV2CpiBuilder::new(mpl_core_program)
		.asset(mint_info)
		.collection(Some(&collection_info))
		.payer(signer)
		.owner(Some(signer))
		.name(collection.base.name.clone())
		.uri(collection.base.uri.clone())
		.authority(Some(collection_authority))
		.plugins(vec![
			PluginAuthorityPair {
				plugin: Plugin::Attributes(attributes_plugin.attributes),
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

  	Ok(())
}