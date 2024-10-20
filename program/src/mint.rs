use forge_api::{
	consts::COAL_UPDATE_AUTHORITY, error::ForgeError, instruction::MintV1Args, loaders::{load_config, load_program, load_signer, load_token_account}, state::Config
};
use forge_utils::{spl::burn, AccountDeserialize};
use solana_program::{
  account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError
};
use mpl_core::{
  instructions::CreateV2CpiBuilder, types::{Attribute, Attributes, Plugin, PluginAuthority, PluginAuthorityPair}, Collection
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
		msg!("Minting ingredient: {:?}, amount: {:?}", ingredient, amount);
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
	let royalties_plugin = collection.plugin_list.royalties.unwrap();
	
	let mut attribute_list = vec![
		Attribute {
			key: "multiplier".to_string(),
			value: "70".to_string(),
		},
		Attribute {
			key: "rarity".to_string(),
			value: "common".to_string(),
		},
	];
	
	match args.resource.as_str() {
		"coal" => {
			attribute_list.push(Attribute {
				key: "resource".to_string(),
				value: "coal".to_string(),
			});
			attribute_list.push(Attribute {
				key: "durability".to_string(),
				value: "1000".to_string(),
			});
		},
		"wood" => {
			attribute_list.push(Attribute {
				key: "resource".to_string(),
				value: "wood".to_string(),
			});
			attribute_list.push(Attribute {
				key: "durability".to_string(),
				value: "100".to_string(),
			});
		},
		_ => {
			return Err(ForgeError::InvalidResource.into());
		}
	};
	let name = match args.resource.as_str() {
		"coal" => "Miner's Pickaxe".to_string(),
		"wood" => "Woodcutter's Axe".to_string(),
		_ => {
			return Err(ForgeError::InvalidResource.into());
		}
	};
	let uri = match args.resource.as_str() {
		"coal" => "https://minechain.gg/metadata.pickaxe.json".to_string(),
		"wood" => "https://minechain.gg/metadata.axe.json".to_string(),
		_ => {
			return Err(ForgeError::InvalidResource.into());
		}
	};

	let collection_authority_seeds = &[b"collection_authority".as_ref(), &[args.collection_authority_bump]];

  	CreateV2CpiBuilder::new(mpl_core_program)
		.asset(mint_info)
		.collection(Some(&collection_info))
		.payer(signer)
		.owner(Some(signer))
		.name(name)
		.uri(uri)
		.authority(Some(collection_authority))
		.plugins(vec![
			PluginAuthorityPair {
				plugin: Plugin::Attributes(Attributes {
					attribute_list
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

  	Ok(())
}